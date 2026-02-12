use crate::{
    error::Error,
    ip2location::{db::LocationDB, record::LocationRecord},
    ip2proxy::{db::ProxyDB, record::ProxyRecord},
};
use memmap2::Mmap;
use std::{
    borrow::Cow,
    net::{IpAddr, Ipv6Addr},
    path::{Path, PathBuf},
};

/// Start of the 6to4 IPv6 address range (`2002::/16`).
pub const FROM_6TO4: u128 = 0x2002_0000_0000_0000_0000_0000_0000_0000;
/// End of the 6to4 IPv6 address range.
pub const TO_6TO4: u128 = 0x2002_ffff_ffff_ffff_ffff_ffff_ffff_ffff;
/// Start of the Teredo IPv6 address range (`2001:0000::/32`).
pub const FROM_TEREDO: u128 = 0x2001_0000_0000_0000_0000_0000_0000_0000;
/// End of the Teredo IPv6 address range.
pub const TO_TEREDO: u128 = 0x2001_0000_ffff_ffff_ffff_ffff_ffff_ffff;

/// A loaded IP2Location or IP2Proxy database.
///
/// Created via [`DB::from_file`]. The underlying BIN file is memory-mapped
/// and remains mapped for the lifetime of this value.
#[derive(Debug)]
pub enum DB {
    /// An IP2Location geolocation database.
    LocationDb(LocationDB),
    /// An IP2Proxy proxy-detection database.
    ProxyDb(ProxyDB),
}

/// A lookup result from either database type.
///
/// The record borrows string data from the memory-mapped file, so it
/// cannot outlive the [`DB`] that produced it.
#[derive(Debug)]
pub enum Record<'a> {
    /// Geolocation record (country, city, coordinates, …).
    LocationDb(Box<LocationRecord<'a>>),
    /// Proxy detection record (proxy type, threat, provider, …).
    ProxyDb(Box<ProxyRecord<'a>>),
}

/// Memory-mapped BIN file backing all read operations.
///
/// All `read_*` methods use **1-based offsets** to match the IP2Location
/// BIN format specification. Bounds are checked on every access.
#[derive(Debug)]
pub(crate) struct Source {
    path: PathBuf,
    map: Mmap,
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl Source {
    /// Wrap an already-mapped file.
    pub fn new(path: PathBuf, map: Mmap) -> Self {
        Self { path, map }
    }

    /// Read a single byte at the given 1-based offset.
    pub fn read_u8(&self, offset: u64) -> Result<u8, Error> {
        if offset == 0 {
            return Err(Error::GenericError("read_u8: offset must be >= 1".into()));
        }
        let idx = (offset - 1) as usize;
        self.map.get(idx).copied().ok_or_else(|| {
            Error::GenericError(format!("read_u8: offset {} out of bounds (len={})", offset, self.map.len()))
        })
    }

    /// Read a little-endian `u32` at the given 1-based offset.
    pub fn read_u32(&self, offset: u64) -> Result<u32, Error> {
        if offset == 0 {
            return Err(Error::GenericError("read_u32: offset must be >= 1".into()));
        }
        let start = (offset - 1) as usize;
        let end = start + 4;
        let slice = self.map.get(start..end).ok_or_else(|| {
            Error::GenericError(format!("read_u32: offset {} out of bounds (len={})", offset, self.map.len()))
        })?;
        Ok(u32::from_le_bytes(slice.try_into()?))
    }

    /// Read a little-endian `f32` at the given 1-based offset.
    pub fn read_f32(&self, offset: u64) -> Result<f32, Error> {
        if offset == 0 {
            return Err(Error::GenericError("read_f32: offset must be >= 1".into()));
        }
        let start = (offset - 1) as usize;
        let end = start + 4;
        let slice = self.map.get(start..end).ok_or_else(|| {
            Error::GenericError(format!("read_f32: offset {} out of bounds (len={})", offset, self.map.len()))
        })?;
        Ok(f32::from_le_bytes(slice.try_into()?))
    }

    /// Read a length-prefixed string at the given 1-based offset.
    ///
    /// The first byte at `offset + 1` is the string length, followed by
    /// that many bytes of content. Returns `Cow::Borrowed` when the bytes
    /// are valid UTF-8 (zero-copy), or `Cow::Owned` with lossy replacement.
    pub fn read_str(&self, offset: u64) -> Result<Cow<'_, str>, Error> {
        let len = self.read_u8(offset + 1)? as usize;
        let start = (offset + 1) as usize;
        let end = start + len;
        if end > self.map.len() {
            return Err(Error::GenericError(format!(
                "read_str: range {}..{} out of bounds (len={})", start, end, self.map.len()
            )));
        }
        let s = String::from_utf8_lossy(&self.map[start..end]);
        Ok(s)
    }

    /// Read a 128-bit IPv6 address stored in reverse byte order at the
    /// given 1-based offset.
    pub fn read_ipv6(&self, offset: u64) -> Result<Ipv6Addr, Error> {
        if offset == 0 {
            return Err(Error::GenericError("read_ipv6: offset must be >= 1".into()));
        }
        let start = (offset - 1) as usize;
        let end = start + 16;
        if end > self.map.len() {
            return Err(Error::GenericError(format!(
                "read_ipv6: range {}..{} out of bounds (len={})", start, end, self.map.len()
            )));
        }
        let mut buf = [0_u8; 16];
        for i in 0..16 {
            buf[i] = self.map[start + 15 - i];
        }
        Ok(Ipv6Addr::from(buf))
    }
}

impl DB {
    /// Consume the unopened db and mmap the file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<DB, Error> {
        //! Loads a Ip2Location/Ip2Proxy Database .bin file from path using
        //! mmap (memap) feature.
        //!
        //! ## Example usage
        //!
        //!```rust
        //! use ip2location::DB;
        //!
        //! let mut db = DB::from_file("data/IP2PROXY-IP-COUNTRY.BIN").unwrap();
        //!```
        if !path.as_ref().exists() {
            return Err(Error::IoError(
                "Error opening DB file: No such file or directory".to_string(),
            ));
        }

        let file = std::fs::File::open(&path)?;
        // SAFETY: The file is opened read-only and we do not modify the
        // mapped region. The caller must ensure the file is not truncated
        // while the DB is in use (standard mmap contract).
        let map = unsafe { Mmap::map(&file) }?;

        // Read product_code (byte 30, 1-indexed) to determine DB type
        if map.len() < 32 {
            return Err(Error::GenericError("DB file too small to contain a valid header".into()));
        }
        let product_code = map[29]; // byte 30, 0-indexed

        let source = Source::new(path.as_ref().to_path_buf(), map);

        match product_code {
            1 => {
                // IP2Location DB
                let mut ldb = LocationDB::new(source);
                ldb.read_header()?;
                Ok(DB::LocationDb(ldb))
            }
            2 => {
                // IP2Proxy DB
                let mut pdb = ProxyDB::new(source);
                pdb.read_header()?;
                Ok(DB::ProxyDb(pdb))
            }
            0 => {
                // Legacy DBs (product_code == 0): try Location first, then Proxy
                let mut ldb = LocationDB::new(source);
                match ldb.read_header() {
                    Ok(()) => Ok(DB::LocationDb(ldb)),
                    Err(_) => {
                        // Re-open and try as Proxy
                        let file = std::fs::File::open(&path)?;
                        // SAFETY: same contract as above.
                        let map = unsafe { Mmap::map(&file) }?;
                        let source = Source::new(path.as_ref().to_path_buf(), map);
                        let mut pdb = ProxyDB::new(source);
                        pdb.read_header()?;
                        Ok(DB::ProxyDb(pdb))
                    }
                }
            }
            _ => Err(Error::UnknownDb),
        }
    }

    pub fn print_db_info(&self) {
        //! Prints the DB Information of Ip2Location/Ip2Proxy to console
        //!
        //! ## Example usage
        //!
        //! ```rust
        //! use ip2location::DB;
        //!
        //! let mut db = DB::from_file("data/IP2LOCATION-LITE-DB1.BIN").unwrap();
        //! db.print_db_info();
        //! ```
        match self {
            Self::LocationDb(db) => db.print_db_info(),
            Self::ProxyDb(db) => db.print_db_info(),
        }
    }

    pub fn ip_lookup(&self, ip: IpAddr) -> Result<Record<'_>, Error> {
        //! Lookup for the given IPv4 or IPv6 and returns the
        //! Geo information or Proxy Information
        //!
        //! ## Example usage
        //!
        //!```rust
        //! use ip2location::{DB, Record};
        //!
        //! let mut db = DB::from_file("data/IP2LOCATION-LITE-DB1.IPV6.BIN").unwrap();
        //! let geo_info = db.ip_lookup("2a01:cb08:8d14::".parse().unwrap()).unwrap();
        //! println!("{:#?}", geo_info);
        //! let record = if let Record::LocationDb(rec) = geo_info {
        //!   Some(rec)
        //! } else { None };
        //! let geo_info = record.unwrap();
        //! assert!(!geo_info.country.is_none());
        //! assert_eq!(geo_info.country.unwrap().short_name, "FR")
        //!```
        match self {
            Self::LocationDb(db) => Ok(Record::LocationDb(Box::new(db.ip_lookup(ip)?))),
            Self::ProxyDb(db) => Ok(Record::ProxyDb(Box::new(db.ip_lookup(ip)?))),
        }
    }
}
