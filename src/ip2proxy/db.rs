use crate::{
    common::{Source, FROM_6TO4, FROM_TEREDO, TO_6TO4, TO_TEREDO},
    error::Error,
    ip2proxy::{
        consts::*,
        record::{Country, Proxy, ProxyRecord},
    },
};
use memmap::Mmap;
use std::{
    borrow::Cow,
    fs::File,
    net::{IpAddr, Ipv6Addr},
    path::Path,
};

#[derive(Debug)]
pub struct ProxyDB {
    //    path: PathBuf,
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
    licence_code: u8,
    product_code: u8,
    database_size: u32,
    source: Source,
}

impl ProxyDB {
    pub(crate) fn new(source: Source) -> Self {
        Self {
            //            path: db.path,
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
            licence_code: 0,
            product_code: 0,
            database_size: 0,
            source,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        //! Loads a Ip2Proxy Database .bin file from path using
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

        let db = File::open(&path)?;
        let map = unsafe { Mmap::map(&db) }?;
        let mut pdb = Self::new(Source::new(path.as_ref().to_path_buf(), map));
        pdb.read_header()?;
        Ok(pdb)
    }

    pub fn ip_lookup(&self, ip: IpAddr) -> Result<ProxyRecord, Error> {
        //! Lookup for the given IPv4 or IPv6 and returns the Proxy information
        //!
        //! ## Example usage
        //!
        //!```rust
        //! use ip2location::{DB, Record};
        //!
        //! let mut db = DB::from_file("data/IP2PROXY-IP-COUNTRY.BIN").unwrap();
        //! let geo_info = db.ip_lookup("1.1.1.1".parse().unwrap()).unwrap();
        //! println!("{:#?}", geo_info);
        //! let record = if let Record::ProxyDb(rec) = geo_info {
        //!   Some(rec)
        //! } else { None };
        //! let geo_info = record.unwrap();
        //! assert!(!geo_info.country.is_none());
        //!```
        match ip {
            IpAddr::V4(ipv4) => {
                let mut record = self.get_ipv4_record(u32::from(ipv4))?;
                record.ip = ip;
                Ok(record)
            }
            IpAddr::V6(ipv6) => {
                if let Some(converted_ip) = ipv6.to_ipv4() {
                    let mut record = self.get_ipv4_record(u32::from(converted_ip))?;
                    record.ip = ip;
                    Ok(record)
                } else if Ipv6Addr::from(FROM_6TO4) <= ipv6 && ipv6 <= Ipv6Addr::from(TO_6TO4) {
                    let ipnum = (u128::from(ipv6) >> 80) as u32;
                    let mut record = self.get_ipv4_record(ipnum)?;
                    record.ip = ip;
                    Ok(record)
                } else if Ipv6Addr::from(FROM_TEREDO) <= ipv6 && ipv6 <= Ipv6Addr::from(TO_TEREDO) {
                    let ipnum = !u128::from(ipv6) as u32;
                    let mut record = self.get_ipv4_record(ipnum)?;
                    record.ip = ip;
                    Ok(record)
                } else {
                    let mut record = self.get_ipv6_record(ipv6)?;
                    record.ip = ip;
                    Ok(record)
                }
            }
        }
    }

    pub fn print_db_info(&self) {
        println!("Db Path: {}", self.source);
        println!(" |- Db Type: {}", self.db_type);
        println!(" |- Db Column: {}", self.db_column);
        println!(
            " |- Db Date (YY/MM/DD): {}/{}/{}",
            self.db_year, self.db_month, self.db_day
        );
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
        self.licence_code = self.source.read_u8(31)?;
        self.database_size = self.source.read_u32(32)?;
        if (self.db_year <= 20 && self.product_code == 0) || self.product_code == 2 {
            Ok(())
        } else {
            Err(Error::InvalidBinDatabase(self.db_year, self.product_code))
        }
    }

    fn get_ipv4_record(&self, mut ip_number: u32) -> Result<ProxyRecord, Error> {
        let mut ip_from: u32;
        let mut ip_to: u32;
        if ip_number == MAX_IPV4_RANGE {
            ip_number -= 1;
        }
        let base_address = self.ipv4_db_addr;
        let database_column = self.db_column;
        let ipv4_index_base_address = self.ipv4_index_base_addr;
        let mut low: u32 = 0;
        let mut high = self.ipv4_db_count;
        let mut mid: u32;
        let column_offset = database_column * 4;
        let mut row_offset: u32;
        let mut mem_offset: u32;
        if ipv4_index_base_address > 0 {
            let number = ip_number >> 16;
            let index_pos = ipv4_index_base_address + (number << 3);
            //            let index_buffer = &mut vec![0_u8; 8];
            mem_offset = index_pos;
            low = self.source.read_u32(mem_offset as u64)?;
            high = self.source.read_u32((mem_offset + 4) as u64)?;
        }
        while low <= high {
            mid = (low + high) >> 1;
            row_offset = base_address + (mid * column_offset as u32);
            mem_offset = row_offset;
            ip_from = self.source.read_u32(mem_offset as u64)?;
            ip_to = self
                .source
                .read_u32(mem_offset as u64 + column_offset as u64)?;
            if ip_number >= ip_from && ip_number < ip_to {
                return self.read_record(mem_offset + 4);
            } else if ip_number < ip_from {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        Err(Error::RecordNotFound)
    }

    fn get_ipv6_record(&self, ip_address: Ipv6Addr) -> Result<ProxyRecord, Error> {
        let base_address = self.ipv6_db_addr;
        let database_column = self.db_column;
        let ipv6_index_base_address = self.ipv4_index_base_addr;
        let mut low = 0_u32;
        let mut high = self.ipv6_db_count;
        let mut mid;
        let mut ip_from: Ipv6Addr;
        let mut ip_to: Ipv6Addr;
        let column_offset = database_column * 4 + 12;
        let mut row_offset;
        let mut mem_offset: u32;
        if high == 0 {
            return Ok(ProxyRecord::default());
        }
        if ipv6_index_base_address > 0 {
            let number = (ip_address.octets()[0] as u32 * 256) + ip_address.octets()[1] as u32;
            let index_pos = ipv6_index_base_address + (number << 3);
            //let index_buffer = &mut vec![0_u8; 8];
            mem_offset = index_pos;
            low = self.source.read_u32(mem_offset as u64)?;
            high = self.source.read_u32((mem_offset + 4) as u64)?;
        }
        while low <= high {
            mid = (low + high) >> 1;
            row_offset = base_address + (mid + column_offset as u32);

            mem_offset = row_offset;
            ip_from = self.source.read_ipv6(mem_offset as u64)?;
            ip_to = self
                .source
                .read_ipv6((mem_offset + column_offset as u32) as u64)?;
            if ip_address > ip_from && ip_address < ip_to {
                return self.read_record(mem_offset + 16);
            } else if ip_address < ip_from {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        Err(Error::RecordNotFound)
    }

    fn read_record(&self, offset: u32) -> Result<ProxyRecord, Error> {
        let db_type = self.db_type as usize;
        let mut record = ProxyRecord::default();

        if REGION_POSITION[db_type] != 0 && record.region.is_none() {
            let index = self
                .source
                .read_u32(4 * (REGION_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.region = Some(self.source.read_str(index as u64)?);
        }
        if CITY_POSITION[db_type] != 0 && record.city.is_none() {
            let index = self
                .source
                .read_u32(4 * (CITY_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.city = Some(self.source.read_str(index as u64)?);
        }
        if ISP_POSITION[db_type] != 0 && record.isp.is_none() {
            let index = self
                .source
                .read_u32(4 * (ISP_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.isp = Some(self.source.read_str(index as u64)?);
        }
        if PROXY_TYPE_POSITION[db_type] != 0 && record.proxy_type.is_none() {
            let index = self
                .source
                .read_u32(4 * (PROXY_TYPE_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.proxy_type = Some(self.source.read_str(index as u64)?);
        }
        if DOMAIN_POSITION[db_type] != 0 && record.domain.is_none() {
            let index = self
                .source
                .read_u32(4 * (DOMAIN_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.domain = Some(self.source.read_str(index as u64)?);
        }
        if USAGE_TYPE_POSITION[db_type] != 0 && record.usage_type.is_none() {
            let index = self
                .source
                .read_u32(4 * (USAGE_TYPE_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.usage_type = Some(self.source.read_str(index as u64)?);
        }
        if ASN_POSITION[db_type] != 0 && record.asn.is_none() {
            let index = self
                .source
                .read_u32(4 * (ASN_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.asn = Some(self.source.read_str(index as u64)?);
        }
        if AS_POSITION[db_type] != 0 && record.as_.is_none() {
            let index = self
                .source
                .read_u32(4 * (AS_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.as_ = Some(self.source.read_str(index as u64)?);
        }
        if LAST_SEEN_POSITION[db_type] != 0 && record.last_seen.is_none() {
            let index = self
                .source
                .read_u32(4 * (LAST_SEEN_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.last_seen = Some(self.source.read_str(index as u64)?);
        }
        if THREAT_POSITION[db_type] != 0 && record.threat.is_none() {
            let index = self
                .source
                .read_u32(4 * (THREAT_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.threat = Some(self.source.read_str(index as u64)?);
        }
        if PROVIDER_POSITION[db_type] != 0 && record.provider.is_none() {
            let index = self
                .source
                .read_u32(4 * (PROVIDER_POSITION[db_type] - 2) as u64 + offset as u64)?;
            record.provider = Some(self.source.read_str(index as u64)?);
        }
        if COUNTRY_POSITION[db_type] != 0 {
            let index = self
                .source
                .read_u32(offset as u64 + 4 * (COUNTRY_POSITION[db_type] - 2) as u64)?;
            let country_short = self.source.read_str(index as u64)?;
            let country_long = self.source.read_str(index as u64 + 3)?;
            if country_short == "-" {
                record.is_proxy = Some(Proxy::IsNotAProxy);
            } else {
                if record.proxy_type.is_none() {
                    let index = self
                        .source
                        .read_u32(4 * (COUNTRY_POSITION[db_type] - 2) as u64 + offset as u64)?;
                    record.proxy_type = Some(self.source.read_str(index as u64)?);
                }
                if record.proxy_type == Some(Cow::from("DCH"))
                    || record.proxy_type == Some(Cow::from("SES"))
                {
                    record.is_proxy = Some(Proxy::IsADataCenterIpAddress);
                } else {
                    record.is_proxy = Some(Proxy::IsAProxy);
                }
            }
            record.country = Some(Country {
                short_name: country_short,
                long_name: country_long,
            });
        }

        Ok(record)
    }
}
