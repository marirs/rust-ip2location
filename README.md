# IP2Location & IP2Proxy

[![Crates.io](https://img.shields.io/crates/v/ip2location)](https://crates.io/crates/ip2location)
[![Documentation](https://docs.rs/ip2location/badge.svg)](https://docs.rs/ip2location/0.3.0/ip2location/struct.DB.html)

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
            "US",
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
---
### Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
ip2location = "0.2.0"
```

### License
This is free software, licensed under the MIT license.

### Ip2Location Databases:
- [Free](https://lite.ip2location.com/)
- [Commercial](https://ip2location.com/database/ip2location)

---
Sriram



