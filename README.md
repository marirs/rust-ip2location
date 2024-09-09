# IP2Location & IP2Proxy

[![Crates.io](https://img.shields.io/crates/v/ip2location)](https://crates.io/crates/ip2location)
[![Documentation](https://docs.rs/ip2location/badge.svg)](https://docs.rs/ip2location/)

[![Linux Arm7](https://github.com/marirs/rust-ip2location/actions/workflows/linux_arm.yml/badge.svg)](https://github.com/marirs/rust-ip2location/actions/workflows/linux_arm.yml)
[![Linux x86_64](https://github.com/marirs/rust-ip2location/actions/workflows/linux_x86_64.yml/badge.svg)](https://github.com/marirs/rust-ip2location/actions/workflows/linux_x86_64.yml)
[![macOS x86_64](https://github.com/marirs/rust-ip2location/actions/workflows/macos.yml/badge.svg)](https://github.com/marirs/rust-ip2location/actions/workflows/macos.yml)
[![Windows x86_64](https://github.com/marirs/rust-ip2location/actions/workflows/windows.yml/badge.svg)](https://github.com/marirs/rust-ip2location/actions/workflows/windows.yml)


This library reads the IP2Location DB format for both IP2Location and IP2Proxy and returns geo information for the given IP.

### Requirements
- `Rust 1.60.0` and above (edition 2021)

### Building
- debug
```bash
cargo b
```
- release
```bash
cargo b --release
```

### Testing
```bash
cargo t -v
```

### Usage
```toml
[dependencies]
ip2location = "0.5.3"
```

### Example
```rust
use ip2location::{error, Record, DB};

const IPV4BIN: &str = "data/IP2LOCATION-LITE-DB1.BIN";
const IPV6BIN: &str = "data/IP2LOCATION-LITE-DB1.IPV6.BIN";
const IP2PROXYBIN: &str = "data/IP2PROXY-IP-COUNTRY.BIN";

// Lookup an IP v4 in the IP2Location V6 BIN Database
fn ip_lookup_in_ipv6bin() -> Result<(), error::Error> {
    let mut db = DB::from_file(IPV6BIN)?;
    let record = db.ip_lookup("43.224.159.155".parse().unwrap())?;
    let record = if let Record::LocationDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    assert!(record.is_some());
    let record = record.unwrap();
    assert!(!record.country.is_none());
    assert_eq!(record.country.clone().unwrap().short_name, "IN");
    assert_eq!(record.country.unwrap().long_name, "India");
    Ok(())
}

// Lookup an IP v4 in the IP2Location V4 BIN Database
fn ip_lookup_in_ipv4bin() -> Result<(), error::Error> {
    let mut db = DB::from_file(IPV4BIN)?;
    let record = db.ip_lookup("43.224.159.155".parse().unwrap())?;
    let record = if let Record::LocationDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    assert!(record.is_some());
    let record = record.unwrap();
    assert!(!record.country.is_none());
    assert_eq!(record.country.clone().unwrap().short_name, "IN");
    assert_eq!(record.country.unwrap().long_name, "India");
    Ok(())
}

// Lookup an IP in the Proxy Database
fn ip_lookup_in_proxy_bin() -> Result<(), error::Error> {
    let mut db = DB::from_file(IP2PROXYBIN)?;
    let record = db.ip_lookup("1.1.1.1".parse().unwrap())?;
    let record = if let Record::ProxyDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    assert!(record.is_some());
    let record = record.unwrap();
    assert!(!record.country.is_none());
    Ok(())
}
```

### Executing the Example
```bash
cargo b --example

# IP2Lcoation Example

./target/debug/examples/lookup data/IP2LOCATION-LITE-DB1.IPV6.BIN 2a01:cb08:8d14::
Db Path: data/IP2LOCATION-LITE-DB1.IPV6.BIN
 |- Db Type: 1
 |- Db Column: 2
 |- Db Date (YY/MM/DD): 20/12/28

Ok(
    Record {
        ip: "2a01:cb08:8d14::",
        latitude: None,
        longitude: None,
        country: Some(
            Country {
                short_name: "FR",
                long_name: "France",
            },
        ),
        region: None,
        city: None,
        isp: None,
        domain: None,
        zip_code: None,
        time_zone: None,
        net_speed: None,
        idd_code: None,
        area_code: None,
        weather_station_code: None,
        weather_station_name: None,
        mcc: None,
        mnc: None,
        mobile_brand: None,
        elevation: None,
        usage_type: None,
        address_type: None,
        category: None,
        district: None,
        asn: None,
        as_name: None,
    },
)

# IP2Proxy Example 
 
./target/debug/examples/lookup data/sample.bin.px11/IP2PROXY-IP-PROXYTYPE-COUNTRY-REGION-CITY-ISP-DOMAIN-USAGETYPE-ASN-LASTSEEN-THREAT-RESIDENTIAL-PROVIDER.BIN 194.59.249.19
Db Path: data/sample.bin.px11/IP2PROXY-IP-PROXYTYPE-COUNTRY-REGION-CITY-ISP-DOMAIN-USAGETYPE-ASN-LASTSEEN-THREAT-RESIDENTIAL-PROVIDER.BIN
 |- Db Type: 11
 |- Db Column: 13
 |- Db Date (YY/MM/DD): 21/5/28

ProxyDb(
    ProxyRecord {
        ip: 1.1.1.1,
        country: Some(
            Country {
                short_name: "US",
                long_name: "United States of America",
            },
        ),
        region: Some(
            "California",
        ),
        city: Some(
            "Los Angeles",
        ),
        isp: Some(
            "APNIC and CloudFlare DNS Resolver Project",
        ),
        domain: Some(
            "cloudflare.com",
        ),
        is_proxy: Some(
            IsAProxy,
        ),
        proxy_type: Some(
            "DCH",
        ),
        asn: Some(
            "13335",
        ),
        as_: Some(
            "CloudFlare Inc",
        ),
        last_seen: Some(
            "27",
        ),
        threat: Some(
            "-",
        ),
        provider: Some(
            "-",
        ),
        usage_type: Some(
            "CDN",
        ),
    },
)
```

### License
This is free software, licensed under the MIT license.

### Ip2Location Databases:
- Lite free version: [Free](https://lite.ip2location.com/)
- Ip2Location / Ip2Proxy: [Commercial](https://ip2location.com/database/)

---
Sriram



