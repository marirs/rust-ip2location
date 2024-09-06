use crate::{
    common::{Source, FROM_6TO4, FROM_TEREDO, TO_6TO4, TO_TEREDO},
    error::Error,
    ip2location::{
        consts::*,
        record::{self, LocationRecord},
    },
};
use memmap::Mmap;
use std::{
    fs::File,
    net::{IpAddr, Ipv6Addr},
    path::Path,
    result::Result,
};

#[derive(Debug)]
pub struct LocationDB {
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
    product_code: u8,
    license_code: u8,
    database_size: u32,
    source: Source,
}

impl LocationDB {
    pub(crate) fn new(source: Source) -> Self {
        Self {
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
            product_code: 0,
            license_code: 0,
            database_size: 0,
            source,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        //! Loads a Ip2Location Database .bin file from path using
        //! mmap (memap) feature.
        //!
        //! ## Example usage
        //!
        //!```rust
        //! use ip2location::DB;
        //!
        //! let mut db = DB::from_file("data/IP2LOCATION-LITE-DB1.BIN").unwrap();
        //!```
        if !path.as_ref().exists() {
            return Err(Error::IoError(
                "Error opening DB file: No such file or directory".to_string(),
            ));
        }

        let db = File::open(&path)?;
        let map = unsafe { Mmap::map(&db) }?;
        let mut ldb = Self::new(Source::new(path.as_ref().to_path_buf(), map));
        ldb.read_header()?;
        Ok(ldb)
    }

    pub fn print_db_info(&self) {
        //! Prints the DB Information to console
        //!
        //! ## Example usage
        //!
        //! ```rust
        //! use ip2location::DB;
        //!
        //! let mut db = DB::from_file("data/IP2LOCATION-LITE-DB1.BIN").unwrap();
        //! db.print_db_info();
        //! ```
        println!("Db Path: {}", self.source);
        println!(" |- Db Type: {}", self.db_type);
        println!(" |- Db Column: {}", self.db_column);
        println!(
            " |- Db Date (YY/MM/DD): {}/{}/{}",
            self.db_year, self.db_month, self.db_day
        );
    }

    pub fn ip_lookup(&self, ip: IpAddr) -> Result<LocationRecord, Error> {
        //! Lookup for the given IPv4 or IPv6 and returns the Geo information
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
        match ip {
            IpAddr::V4(ipv4) => {
                let mut record = self.ipv4_lookup(u32::from(ipv4))?;
                record.ip = ip;
                Ok(record)
            }
            IpAddr::V6(ipv6) => {
                if let Some(converted_ip) = ipv6.to_ipv4() {
                    let mut record = self.ipv4_lookup(u32::from(converted_ip))?;
                    record.ip = ip;
                    Ok(record)
                } else if Ipv6Addr::from(FROM_6TO4) <= ipv6 && ipv6 <= Ipv6Addr::from(TO_6TO4) {
                    let ipnum = (u128::from(ipv6) >> 80) as u32;
                    let mut record = self.ipv4_lookup(ipnum)?;
                    record.ip = ip;
                    Ok(record)
                } else if Ipv6Addr::from(FROM_TEREDO) <= ipv6 && ipv6 <= Ipv6Addr::from(TO_TEREDO) {
                    let ipnum = !u128::from(ipv6) as u32;
                    let mut record = self.ipv4_lookup(ipnum)?;
                    record.ip = ip;
                    Ok(record)
                } else {
                    let mut record = self.ipv6_lookup(ipv6)?;
                    record.ip = ip;
                    Ok(record)
                }
            }
        }
    }

    fn read_header(&mut self) -> Result<(), Error> {
        self.db_type = self.source.read_u8(1)?;
        self.db_column = self.source.read_u8(2)?;
        self.db_year = self.source.read_u8(3)?;
        self.db_month = self.source.read_u8(4)?;
        self.db_day = self.source.read_u8(5)?;
        self.ipv4_db_count = self.source.read_u32(6)?;
        self.ipv4_db_addr = self.source.read_u32(10)?;
        self.ipv6_db_count = self.source.read_u32(14)?;
        self.ipv6_db_addr = self.source.read_u32(18)?;
        self.ipv4_index_base_addr = self.source.read_u32(22)?;
        self.ipv6_index_base_addr = self.source.read_u32(26)?;
        self.product_code = self.source.read_u8(30)?;
        self.license_code = self.source.read_u8(31)?;
        self.database_size = self.source.read_u32(32)?;
        if (self.db_year <= 20 && self.product_code == 0) || self.product_code == 1 {
            Ok(())
        } else {
            Err(Error::InvalidBinDatabase(self.db_year, self.product_code))
        }
    }

    fn ipv4_lookup(&self, mut ip_number: u32) -> Result<LocationRecord, Error> {
        if ip_number == u32::MAX {
            ip_number -= 1;
        }
        let mut low = 0;
        let mut high = self.ipv4_db_count;
        if self.ipv4_index_base_addr > 0 {
            let index = ((ip_number >> 16) << 3) + self.ipv4_index_base_addr;
            low = self.source.read_u32(index as u64)?;
            high = self.source.read_u32((index + 4) as u64)?;
        }
        while low <= high {
            let mid = (low + high) >> 1;
            let ip_from = self
                .source
                .read_u32((self.ipv4_db_addr + mid * (self.db_column as u32) * 4) as u64)?;
            let ip_to = self
                .source
                .read_u32((self.ipv4_db_addr + (mid + 1) * (self.db_column as u32) * 4) as u64)?;
            if (ip_number >= ip_from) && (ip_number < ip_to) {
                return self.read_record(self.ipv4_db_addr + mid * (self.db_column as u32) * 4);
            } else if ip_number < ip_from {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        Err(Error::RecordNotFound)
    }

    fn ipv6_lookup(&self, ipv6: Ipv6Addr) -> Result<LocationRecord, Error> {
        let mut low = 0;
        let mut high = self.ipv6_db_count;
        if self.ipv6_index_base_addr > 0 {
            let num = (ipv6.octets()[0] as u32) * 256 + (ipv6.octets()[1] as u32);
            let index = (num << 3) + self.ipv6_index_base_addr;
            low = self.source.read_u32(index as u64)?;
            high = self.source.read_u32((index + 4) as u64)?;
        }
        while low <= high {
            let mid = (low + high) >> 1;
            let ip_from = self
                .source
                .read_ipv6((self.ipv6_db_addr + mid * ((self.db_column as u32) * 4 + 12)) as u64)?;
            let ip_to = self.source.read_ipv6(
                (self.ipv6_db_addr + (mid + 1) * ((self.db_column as u32) * 4 + 12)) as u64,
            )?;
            if (ipv6 >= ip_from) && (ipv6 < ip_to) {
                return self.read_record(
                    self.ipv6_db_addr + mid * ((self.db_column as u32) * 4 + 12) + 12,
                );
            } else if ipv6 < ip_from {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        Err(Error::RecordNotFound)
    }

    fn read_record(&self, row_addr: u32) -> Result<LocationRecord, Error> {
        let mut result = LocationRecord::default();

        if COUNTRY_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (COUNTRY_POSITION[self.db_type as usize] - 1)).into())?;
            let short_name = self.source.read_str(index.into())?;
            let long_name = self.source.read_str((index + 3).into())?;
            result.country = Some(record::Country {
                short_name,
                long_name,
            });
        }

        if REGION_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (REGION_POSITION[self.db_type as usize] - 1)).into())?;
            result.region = Some(self.source.read_str(index.into())?);
        }

        if LATITUDE_POSITION[self.db_type as usize] > 0 {
            let index = row_addr + 4 * (LATITUDE_POSITION[self.db_type as usize] - 1);
            result.latitude = Some(self.source.read_f32(index.into())?);
        }

        if LONGITUDE_POSITION[self.db_type as usize] > 0 {
            let index = row_addr + 4 * (LONGITUDE_POSITION[self.db_type as usize] - 1);
            result.longitude = Some(self.source.read_f32(index.into())?);
        }

        if CITY_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (CITY_POSITION[self.db_type as usize] - 1)).into())?;
            result.city = Some(self.source.read_str(index.into())?);
        }

        if ISP_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (ISP_POSITION[self.db_type as usize] - 1)).into())?;
            result.isp = Some(self.source.read_str(index.into())?);
        }

        if DOMAIN_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (DOMAIN_POSITION[self.db_type as usize] - 1)).into())?;
            result.domain = Some(self.source.read_str(index.into())?);
        }

        if ZIPCODE_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (ZIPCODE_POSITION[self.db_type as usize] - 1)).into())?;
            result.zip_code = Some(self.source.read_str(index.into())?);
        }

        if TIMEZONE_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (TIMEZONE_POSITION[self.db_type as usize] - 1)).into())?;
            result.time_zone = Some(self.source.read_str(index.into())?);
        }

        if NETSPEED_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (NETSPEED_POSITION[self.db_type as usize] - 1)).into())?;
            result.net_speed = Some(self.source.read_str(index.into())?);
        }

        if IDDCODE_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (IDDCODE_POSITION[self.db_type as usize] - 1)).into())?;
            result.idd_code = Some(self.source.read_str(index.into())?);
        }

        if AREACODE_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (AREACODE_POSITION[self.db_type as usize] - 1)).into())?;
            result.area_code = Some(self.source.read_str(index.into())?);
        }

        if WEATHERSTATIONCODE_POSITION[self.db_type as usize] > 0 {
            let index = self.source.read_u32(
                (row_addr + 4 * (WEATHERSTATIONCODE_POSITION[self.db_type as usize] - 1)).into(),
            )?;
            result.weather_station_code = Some(self.source.read_str(index.into())?);
        }

        if WEATHERSTATIONNAME_POSITION[self.db_type as usize] > 0 {
            let index = self.source.read_u32(
                (row_addr + 4 * (WEATHERSTATIONNAME_POSITION[self.db_type as usize] - 1)).into(),
            )?;
            result.weather_station_name = Some(self.source.read_str(index.into())?);
        }

        if MCC_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (MCC_POSITION[self.db_type as usize] - 1)).into())?;
            result.mcc = Some(self.source.read_str(index.into())?);
        }

        if MNC_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (MNC_POSITION[self.db_type as usize] - 1)).into())?;
            result.mnc = Some(self.source.read_str(index.into())?);
        }

        if MOBILEBRAND_POSITION[self.db_type as usize] > 0 {
            let index = self.source.read_u32(
                (row_addr + 4 * (MOBILEBRAND_POSITION[self.db_type as usize] - 1)).into(),
            )?;
            result.mobile_brand = Some(self.source.read_str(index.into())?);
        }

        if ELEVATION_POSITION[self.db_type as usize] > 0 {
            let index = self.source.read_u32(
                (row_addr + 4 * (ELEVATION_POSITION[self.db_type as usize] - 1)).into(),
            )?;
            result.elevation = Some(self.source.read_str(index.into())?);
        }

        if USAGETYPE_POSITION[self.db_type as usize] > 0 {
            let index = self.source.read_u32(
                (row_addr + 4 * (USAGETYPE_POSITION[self.db_type as usize] - 1)).into(),
            )?;
            result.usage_type = Some(self.source.read_str(index.into())?);
        }

        if ADDRESSTYPE_POSITION[self.db_type as usize] > 0 {
            let index = self.source.read_u32(
                (row_addr + 4 * (ADDRESSTYPE_POSITION[self.db_type as usize] - 1)).into(),
            )?;
            result.address_type = Some(self.source.read_str(index.into())?);
        }

        if CATEGORY_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (CATEGORY_POSITION[self.db_type as usize] - 1)).into())?;
            result.category = Some(self.source.read_str(index.into())?);
        }

        if DISTRICT_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (DISTRICT_POSITION[self.db_type as usize] - 1)).into())?;
            result.district = Some(self.source.read_str(index.into())?);
        }

        if ASN_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (ASN_POSITION[self.db_type as usize] - 1)).into())?;
            result.asn = Some(self.source.read_str(index.into())?);
        }

        if AS_POSITION[self.db_type as usize] > 0 {
            let index = self
                .source
                .read_u32((row_addr + 4 * (AS_POSITION[self.db_type as usize] - 1)).into())?;
            result.as_name = Some(self.source.read_str(index.into())?);
        }
        Ok(result)
    }
}
