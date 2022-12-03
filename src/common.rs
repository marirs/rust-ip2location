use crate::{
    error::Error,
    ip2location::{db::LocationDB, record::Record as LdbRecord},
    ip2proxy::{db::ProxyDB, record::Record as PdbRecord},
};
use memmap::Mmap;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    net::{IpAddr, Ipv6Addr},
    path::{Path, PathBuf},
};

// Constants for IPV6 Address
pub const FROM_6TO4: u128 = 0x2002_0000_0000_0000_0000_0000_0000_0000;
pub const TO_6TO4: u128 = 0x2002_ffff_ffff_ffff_ffff_ffff_ffff_ffff;
pub const FROM_TEREDO: u128 = 0x2001_0000_0000_0000_0000_0000_0000_0000;
pub const TO_TEREDO: u128 = 0x2001_0000_ffff_ffff_ffff_ffff_ffff_ffff;

#[derive(Debug)]
pub enum DB {
    LocationDb(LocationDB),
    ProxyDb(ProxyDB),
}

#[derive(Debug)]
pub enum Record {
    LocationDb(LdbRecord),
    ProxyDb(PdbRecord),
}

#[derive(Debug)]
pub(crate) enum Source {
    File(PathBuf, File),
    Mmap(PathBuf, Mmap),
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Mmap(ff, _) => write!(f, "{}", ff.display()),
            Self::File(ff, _) => write!(f, "{}", ff.display()),
        }
    }
}

impl Source {
    pub fn read_u8(&mut self, offset: u64) -> Result<u8, Error> {
        match self {
            Source::File(_, f) => {
                f.seek(SeekFrom::Start(offset - 1))?;
                let mut buf = [0_u8; 1];
                f.read(&mut buf)?;
                Ok(buf[0])
            }
            Source::Mmap(_, m) => Ok(m[(offset - 1) as usize]),
        }
    }

    pub fn read_u32(&mut self, offset: u64) -> Result<u32, Error> {
        match self {
            Source::File(_, f) => {
                f.seek(SeekFrom::Start(offset - 1))?;
                let mut buf = [0_u8; 4];
                f.read(&mut buf)?;
                let result = u32::from_ne_bytes(buf);
                Ok(result)
            }
            Source::Mmap(_, m) => {
                let mut buf = [0_u8; 4];
                buf[0] = m[(offset - 1) as usize];
                buf[1] = m[offset as usize];
                buf[2] = m[(offset + 1) as usize];
                buf[3] = m[(offset + 2) as usize];
                let result = u32::from_ne_bytes(buf);
                Ok(result)
            }
        }
    }

    pub fn read_f32(&mut self, offset: u64) -> Result<f32, Error> {
        match self {
            Source::File(_, f) => {
                f.seek(SeekFrom::Start(offset - 1))?;
                let mut buf = [0_u8; 4];
                f.read(&mut buf)?;
                let result = f32::from_ne_bytes(buf);
                Ok(result)
            }
            Source::Mmap(_, m) => {
                let mut buf = [0_u8; 4];
                buf[0] = m[(offset - 1) as usize];
                buf[1] = m[offset as usize];
                buf[2] = m[(offset + 1) as usize];
                buf[3] = m[(offset + 2) as usize];
                let result = f32::from_ne_bytes(buf);
                Ok(result)
            }
        }
    }

    pub fn read_str(&mut self, offset: u64) -> Result<String, Error> {
        let len = self.read_u8(offset + 1)? as usize;
        match self {
            Source::File(_, f) => {
                f.seek(SeekFrom::Start(offset + 1))?;
                let mut buf = vec![0_u8; len];
                f.read(&mut buf)?;
                let s = String::from_utf8(buf)?;
                Ok(s)
            }
            Source::Mmap(_, m) => {
                let mut buf = vec![0_u8; len];
                for i in 0..len {
                    buf[i] = m[(offset + 1) as usize + i];
                }
                let s = String::from_utf8(buf)?;
                Ok(s)
            }
        }
    }

    pub fn read_ipv6(&mut self, offset: u64) -> Result<Ipv6Addr, Error> {
        let mut buf = [0_u8; 16];
        let mut i = 0;
        let mut j = 15;
        while i < 16 {
            buf[i] = self.read_u8(offset + j)?;
            i += 1;
            if j > 0 {
                j -= 1;
            }
        }
        let result = Ipv6Addr::from(buf);
        Ok(result)
    }
}

impl DB {
    /// Consume the unopened db and open the file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<DB, Error> {
        if let Ok(location_db) = LocationDB::from_file(&path) {
            Ok(DB::LocationDb(location_db))
        } else if let Ok(proxy_db) = ProxyDB::from_file(&path) {
            Ok(DB::ProxyDb(proxy_db))
        } else {
            Err(Error::UnknownDb)
        }
    }

    /// Consume the unopened db and mmap the file.
    pub fn from_file_mmap<P: AsRef<Path>>(path: P) -> Result<DB, Error> {
        if let Ok(location_db) = LocationDB::from_file_mmap(&path) {
            Ok(DB::LocationDb(location_db))
        } else if let Ok(proxy_db) = ProxyDB::from_file_mmap(&path) {
            Ok(DB::ProxyDb(proxy_db))
        } else {
            Err(Error::UnknownDb)
        }
    }

    pub fn print_db_info(&self) {
        match self {
            Self::LocationDb(db) => db.print_db_info(),
            Self::ProxyDb(db) => db.print_db_info(),
        }
    }

    pub fn ip_lookup(&mut self, ip: IpAddr) -> Result<Record, Error> {
        match self {
            Self::LocationDb(db) => Ok(Record::LocationDb(db.ip_lookup(ip)?)),
            Self::ProxyDb(db) => Ok(Record::ProxyDb(db.ip_lookup(ip)?)),
        }
    }
}
