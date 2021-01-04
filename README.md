Rust IP2Location
-----------------
[![Build Status](https://travis-ci.com/marirs/rust-ip2location.svg?branch=main)](https://travis-ci.com/marirs/rust-ip2location)

This library reads the IP2Location DB format and returns geo information for the given IP.

### Requirements
- `Rust 1.30.0` and above

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

./target/debug/examples/lookup data/IP2LOCATION-LITE-DB1.IPV6.BIN 2a01:cb08:8d14::
Db Path: data/IP2LOCATION-LITE-DB1.IPV6.BIN
 |- Db Type: 1
 |- Db Column: 2
 |- Db Date (YY/MM/DD): 20/12/28
 |- IPv4 Count: 188687
 |- IPv4 Address: 1048641
 |- IPv4 Index Base Address: 65
 |- IPv6 Count: 138876
 |- IPv6 Address: 2558137
 |- IPv6 Index Base Address: 524353

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
    },
)
```
---
### Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
ip2location = "0.1.0"
```

### License
This is free software, licensed under the MIT license.

---
Sriram



