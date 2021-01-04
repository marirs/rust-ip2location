use std::{
    fs::File,
    path::Path,
    io::{Read, Seek, SeekFrom},
    net::{Ipv4Addr, Ipv6Addr},
    result::Result,
    str::FromStr,
};

use memmap::Mmap;

mod consts;

pub mod error;

pub mod record;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct DB {
    path: String,
    db_type: u8,
    db_column: u8,
    db_year: u8,
    db_month: u8,
    db_day: u8,
    ipv4_db_count: u32,
    ipv4_db_addr: u32,
    ipv6_db_count: u32,
    ipv6_db_addr: u32,
    ipv4_index_base_addr: u32,
    ipv6_index_base_addr: u32,
    file: Option<File>,
    mmap: Option<Mmap>,
}

impl DB {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, error::Error> {
        //! Loads a Ip2Location Database .bin file from path
        //!
        //! ## Example usage
        //!
        //!```
        //! use ip2location::DB;
        //!
        //! fn main () {
        //!     let mut db = DB::from_file("data/IP2LOCATION-LITE-DB1.BIN").unwrap();
        //! }
        //!```
        let mut obj = Self::empty();
        obj.path = path
            .as_ref()
            .to_string_lossy()
            .parse()
            .unwrap();
        obj.open()?;
        obj.read_header()?;
        Ok(obj)
    }

    pub fn from_file_mmap<P: AsRef<Path>>(path: P) -> Result<Self, error::Error> {
        //! Loads a Ip2Location Database .bin file from path using
        //! mmap (memap) feature.
        //!
        //! ## Example usage
        //!
        //!```
        //! use ip2location::DB;
        //!
        //! fn main () {
        //!     let mut db = DB::from_file_mmap("data/IP2LOCATION-LITE-DB1.BIN").unwrap();
        //! }
        //!```
        let mut obj = Self::empty();
        obj.path = path
            .as_ref()
            .to_string_lossy()
            .parse()
            .unwrap();
        obj.mmap()?;
        obj.read_header()?;
        Ok(obj)
    }

    pub fn print_db_info(&self) {
        //! Prints the DB Information to console
        //!
        //! ## Example usage
        //!
        //! ```
        //! use ip2location::DB;
        //!
        //! fn main () {
        //!     let mut db = DB::from_file_mmap("data/IP2LOCATION-LITE-DB1.BIN").unwrap();
        //!     db.print_db_info();
        //! }
        //! ```
        println!("Db Path: {}", self.path);
        println!(" |- Db Type: {}", self.db_type);
        println!(" |- Db Column: {}", self.db_column);
        println!(" |- Db Date (YY/MM/DD): {}/{}/{}", self.db_year, self.db_month, self.db_day);
        println!(" |- IPv4 Count: {}", self.ipv4_db_count);
        println!(" |- IPv4 Address: {}", self.ipv4_db_addr);
        println!(" |- IPv4 Index Base Address: {}", self.ipv4_index_base_addr);
        println!(" |- IPv6 Count: {}", self.ipv6_db_count);
        println!(" |- IPv6 Address: {}", self.ipv6_db_addr);
        println!(" |- IPv6 Index Base Address: {}", self.ipv6_index_base_addr);
    }

    pub fn ip_lookup(&mut self, ip: &str) -> Result<record::Record, error::Error> {
        //! Lookup for the given IPv4 or IPv6 and returns the Geo information
        //!
        //! ## Example usage
        //!
        //!```
        //! use ip2location::DB;
        //!
        //! fn main () {
        //!     let mut db = DB::from_file("data/IP2LOCATION-LITE-DB1.IPV6.BIN").unwrap();
        //!     let geo_info = db.ip_lookup("2a01:cb08:8d14::").unwrap();
        //!     println!("{:#?}", geo_info);
        //!     assert!(!geo_info.country.is_none());
        //!     assert_eq!(geo_info.country.unwrap().short_name, "FR")
        //! }
        //!```
        if Self::is_ipv4(ip) {
            let ip_number = u32::from(Ipv4Addr::from_str(ip)?);
            let mut record = self.ipv4_lookup(ip_number)?;
            record.ip = ip.into();
            return Ok(record);
        } else if Self::is_ipv6(ip) {
            let ipv6 = Ipv6Addr::from_str(ip)?;
            let mut record = self.ipv6_lookup(ipv6)?;
            record.ip = ip.into();
            return Ok(record);
        }
        Err(error::Error::InvalidIP("ip address is invalid".into()))
    }

    fn empty() -> Self {
        Self {
            path: "".into(),
            db_type: 0,
            db_column: 0,
            db_year: 0,
            db_month: 0,
            db_day: 0,
            ipv4_db_count: 0,
            ipv4_db_addr: 0,
            ipv6_db_count: 0,
            ipv6_db_addr: 0,
            ipv4_index_base_addr: 0,
            ipv6_index_base_addr: 0,
            file: None,
            mmap: None,
        }
    }

    fn open(&mut self) -> Result<(), error::Error> {
        match File::open(self.path.clone()) {
            Ok(f) => {
                self.file = Some(f);
                return Ok(());
            },
            Err(e) => return Err(error::Error::IoError(format!(
                "Error opening DB file: {}", e
            ))),
        };
    }

    fn mmap(&mut self) -> Result<(), error::Error> {
        match File::open(self.path.clone()) {
            Ok(f) => {
                match unsafe { Mmap::map(&f)  } {
                    Ok(mmap) => {
                        self.mmap= Some(mmap);
                        return Ok(());
                    },
                    Err(e) => return Err(error::Error::IoError(format!(
                        "error while mmaping db file: {}", e
                    ))),
                };
            },
            Err(e) => return Err(error::Error::IoError(format!(
                "Error opening DB file: {}", e
            ))),
        };
    }

    fn read_header(&mut self) -> Result<(), error::Error> {
        self.db_type = self.read_u8(1)?;
        self.db_column = self.read_u8(2)?;
        self.db_year = self.read_u8(3)?;
        self.db_month = self.read_u8(4)?;
        self.db_day = self.read_u8(5)?;
        self.ipv4_db_count = self.read_u32(6)?;
        self.ipv4_db_addr = self.read_u32(10)?;
        self.ipv6_db_count = self.read_u32(14)?;
        self.ipv6_db_addr = self.read_u32(18)?;
        self.ipv4_index_base_addr = self.read_u32(22)?;
        self.ipv6_index_base_addr = self.read_u32(26)?;
        Ok(())
    }

    fn ipv4_lookup(&mut self, mut ip_number: u32) -> Result<record::Record, error::Error> {
        if ip_number == u32::MAX {
            ip_number = ip_number - 1;
        }
        let mut low = 0;
        let mut high = self.ipv4_db_count;
        if self.ipv4_index_base_addr > 0 {
            let index = ((ip_number >> 16) << 3) + self.ipv4_index_base_addr;
            low = self.read_u32(index as u64)?;
            high = self.read_u32((index + 4) as u64)?;
        }
        while low < high {
            let mid = (low + high) >> 1;
            let ip_from = self.read_u32((self.ipv4_db_addr + mid * (self.db_column as u32) * 4) as u64)?;
            let ip_to = self.read_u32((self.ipv4_db_addr + (mid + 1) * (self.db_column as u32) * 4) as u64)?;
            if (ip_number >= ip_from) && (ip_number < ip_to) {
                return self.read_record(self.ipv4_db_addr + mid * (self.db_column as u32) * 4);
            } else {
                if ip_number < ip_from {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
        }
        Err("no record found".into())
    }

    fn ipv6_lookup(&mut self, ipv6: Ipv6Addr) -> Result<record::Record, error::Error> {
        let mut low = 0;
        let mut high = self.ipv6_db_count;
        if self.ipv6_index_base_addr > 0 {
            let num = (ipv6.octets()[0] as u32) * 256 + (ipv6.octets()[1] as u32);
            let index = (num << 3) + self.ipv6_index_base_addr;
            low = self.read_u32(index as u64)?;
            high = self.read_u32((index + 4) as u64)?;
        }
        while low < high {
            let mid = (low + high) >> 1;
            let ip_from = self.read_ipv6((self.ipv6_db_addr + mid * ((self.db_column as u32) * 4 + 12)) as u64)?;
            let ip_to = self.read_ipv6((self.ipv6_db_addr + (mid + 1) * ((self.db_column as u32) * 4 + 12)) as u64)?;
            if (ipv6 >= ip_from) && (ipv6 < ip_to) {
                return self.read_record(self.ipv6_db_addr + mid * ((self.db_column as u32) * 4 + 12) + 12);
            } else {
                if ipv6 < ip_from {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
        }
        Err("no record found".into())
    }

    fn read_record(&mut self, row_addr: u32) -> Result<record::Record, error::Error> {
        let mut result = record::Record::new_empty();

        if consts::COUNTRY_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::COUNTRY_POSITION[self.db_type as usize] - 1)).into())?;
            let short_name = self.read_str(index.into())?;
            let long_name = self.read_str((index + 3).into())?;
            result.country = Some(record::Country { short_name, long_name });
        }

        if consts::REGION_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::REGION_POSITION[self.db_type as usize] - 1)).into())?;
            result.region = Some(self.read_str(index.into())?);
        }

        if consts::LATITUDE_POSITION[self.db_type as usize] > 0 {
            let index = row_addr + 4 * (consts::LATITUDE_POSITION[self.db_type as usize] - 1);
            result.latitude = Some(self.read_f32(index.into())?);
        }

        if consts::LONGITUDE_POSITION[self.db_type as usize] > 0 {
            let index = row_addr + 4 * (consts::LONGITUDE_POSITION[self.db_type as usize] - 1);
            result.longitude = Some(self.read_f32(index.into())?);
        }

        if consts::CITY_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::CITY_POSITION[self.db_type as usize] - 1)).into())?;
            result.city = Some(self.read_str(index.into())?);
        }

        if consts::ISP_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::ISP_POSITION[self.db_type as usize] - 1)).into())?;
            result.isp = Some(self.read_str(index.into())?);
        }

        if consts::DOMAIN_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::DOMAIN_POSITION[self.db_type as usize] - 1)).into())?;
            result.domain = Some(self.read_str(index.into())?);
        }

        if consts::ZIPCODE_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::ZIPCODE_POSITION[self.db_type as usize] - 1)).into())?;
            result.zip_code = Some(self.read_str(index.into())?);
        }

        if consts::TIMEZONE_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::TIMEZONE_POSITION[self.db_type as usize] - 1)).into())?;
            result.time_zone = Some(self.read_str(index.into())?);
        }

        if consts::NETSPEED_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::NETSPEED_POSITION[self.db_type as usize] - 1)).into())?;
            result.net_speed = Some(self.read_str(index.into())?);
        }

        if consts::IDDCODE_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::IDDCODE_POSITION[self.db_type as usize] - 1)).into())?;
            result.idd_code = Some(self.read_str(index.into())?);
        }

        if consts::AREACODE_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::AREACODE_POSITION[self.db_type as usize] - 1)).into())?;
            result.area_code = Some(self.read_str(index.into())?);
        }

        if consts::WEATHERSTATIONCODE_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::WEATHERSTATIONCODE_POSITION[self.db_type as usize] - 1)).into())?;
            result.weather_station_code = Some(self.read_str(index.into())?);
        }

        if consts::WEATHERSTATIONNAME_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::WEATHERSTATIONNAME_POSITION[self.db_type as usize] - 1)).into())?;
            result.weather_station_name = Some(self.read_str(index.into())?);
        }

        if consts::MCC_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::MCC_POSITION[self.db_type as usize] - 1)).into())?;
            result.mcc = Some(self.read_str(index.into())?);
        }

        if consts::MNC_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::MNC_POSITION[self.db_type as usize] - 1)).into())?;
            result.mnc = Some(self.read_str(index.into())?);
        }

        if consts::MOBILEBRAND_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::MOBILEBRAND_POSITION[self.db_type as usize] - 1)).into())?;
            result.mobile_brand = Some(self.read_str(index.into())?);
        }

        if consts::ELEVATION_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::ELEVATION_POSITION[self.db_type as usize] - 1)).into())?;
            result.elevation = Some(self.read_str(index.into())?);
        }

        if consts::USAGETYPE_POSITION[self.db_type as usize] > 0 {
            let index = self.read_u32((row_addr + 4 * (consts::USAGETYPE_POSITION[self.db_type as usize] - 1)).into())?;
            result.usage_type = Some(self.read_str(index.into())?);
        }

        Ok(result)
    }

    fn read_u8(&mut self, offset: u64) -> Result<u8, error::Error> {
        if self.file.is_some() {
            let mut f = self.file.as_ref().unwrap();
            f.seek(SeekFrom::Start(offset - 1))?;
            let mut buf = [0 as u8; 1];
            f.read(&mut buf)?;
            Ok(buf[0])
        } else if self.mmap.is_some() {
            let m = self.mmap.as_ref().unwrap();
            Ok(m[(offset - 1) as usize])
        } else {
            Err(error::Error::InvalidState("db is not open".into()))
        }
    }

    fn read_u32(&mut self, offset: u64) -> Result<u32, error::Error> {
        if self.file.is_some() {
            let mut f = self.file.as_ref().unwrap();
            f.seek(SeekFrom::Start(offset - 1))?;
            let mut buf = [0 as u8; 4];
            f.read(&mut buf)?;
            let result = u32::from_ne_bytes(buf);
            Ok(result)
        } else if self.mmap.is_some() {
            let m = self.mmap.as_ref().unwrap();
            let mut buf = [0 as u8; 4];
            buf[0] = m[(offset - 1) as usize];
            buf[1] = m[offset as usize];
            buf[2] = m[(offset + 1) as usize];
            buf[3] = m[(offset + 2) as usize];
            let result = u32::from_ne_bytes(buf);
            Ok(result)
        } else {
            Err(error::Error::InvalidState("db is not open".into()))
        }
    }

    fn read_f32(&mut self, offset: u64) -> Result<f32, error::Error> {
        if self.file.is_some() {
            let mut f = self.file.as_ref().unwrap();
            f.seek(SeekFrom::Start(offset - 1))?;
            let mut buf = [0 as u8; 4];
            f.read(&mut buf)?;
            let result = f32::from_ne_bytes(buf);
            Ok(result)
        } else if self.mmap.is_some() {
            let m = self.mmap.as_ref().unwrap();
            let mut buf = [0 as u8; 4];
            buf[0] = m[(offset - 1) as usize];
            buf[1] = m[offset as usize];
            buf[2] = m[(offset + 1) as usize];
            buf[3] = m[(offset + 2) as usize];
            let result = f32::from_ne_bytes(buf);
            Ok(result)
        } else {
            Err(error::Error::InvalidState("db is not open".into()))
        }
    }

    fn read_str(&mut self, offset: u64) -> Result<String, error::Error> {
        if self.file.is_some() {
            let len = self.read_u8(offset + 1)? as usize;
            let mut f = self.file.as_ref().unwrap();
            f.seek(SeekFrom::Start(offset + 1))?;
            let mut buf = vec![0 as u8; len];
            f.read(&mut buf)?;
            let s = std::str::from_utf8(&buf)?;
            Ok(s.into())
        } else if self.mmap.is_some() {
            let len = self.read_u8(offset + 1)? as usize;
            let m = self.mmap.as_ref().unwrap();
            let mut buf = vec![0 as u8; len];
            for i in 0..len {
                buf[i] = m[(offset + 1) as usize + i];
            }
            let s = std::str::from_utf8(&buf)?;
            Ok(s.into())
        } else {
            Err(error::Error::InvalidState("db is not open".into()))
        }
    }

    fn read_ipv6(&mut self, offset: u64) -> Result<Ipv6Addr, error::Error> {
        let mut buf = [0 as u8; 16];
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

    fn is_ipv4(ip: &str) -> bool {
        match Ipv4Addr::from_str(ip) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
    }

    fn is_ipv6(ip: &str) -> bool {
        match Ipv6Addr::from_str(ip) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
    }
}
