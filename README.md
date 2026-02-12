# IP2Location & IP2Proxy

[![Crates.io](https://img.shields.io/crates/v/ip2location)](https://crates.io/crates/ip2location)
[![Documentation](https://docs.rs/ip2location/badge.svg)](https://docs.rs/ip2location/)

[![Linux Arm7](https://github.com/marirs/rust-ip2location/actions/workflows/linux_arm.yml/badge.svg)](https://github.com/marirs/rust-ip2location/actions/workflows/linux_arm.yml)
[![Linux x86_64](https://github.com/marirs/rust-ip2location/actions/workflows/linux_x86_64.yml/badge.svg)](https://github.com/marirs/rust-ip2location/actions/workflows/linux_x86_64.yml)
[![macOS x86_64](https://github.com/marirs/rust-ip2location/actions/workflows/macos.yml/badge.svg)](https://github.com/marirs/rust-ip2location/actions/workflows/macos.yml)
[![Windows x86_64](https://github.com/marirs/rust-ip2location/actions/workflows/windows.yml/badge.svg)](https://github.com/marirs/rust-ip2location/actions/workflows/windows.yml)


This library reads the IP2Location DB format for both IP2Location and IP2Proxy and returns geo information for the given IP.

### Features
- **Zero-copy** — string fields borrow directly from the memory-mapped file
- **Memory-mapped I/O** — database is mmap’d at open time, no heap allocation on the lookup path
- Supports both **IP2Location** (geolocation) and **IP2Proxy** (proxy detection) BIN databases
- Handles **IPv4**, **IPv6**, **6to4**, **Teredo**, and **IPv4-mapped IPv6** addresses

### Requirements
- Rust **1.85+** (edition 2024)

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
ip2location = "0.6"
```

### Example
```rust
use ip2location::{error, Record, DB};

const IPV4BIN: &str = "data/IP2LOCATION-LITE-DB1.BIN";
const IPV6BIN: &str = "data/IP2LOCATION-LITE-DB1.IPV6.BIN";
const IP2PROXYBIN: &str = "data/IP2PROXY-IP-COUNTRY.BIN";

// Lookup an IPv4 in the IP2Location IPv6 BIN Database
fn ip_lookup_in_ipv6bin() -> Result<(), error::Error> {
    let db = DB::from_file(IPV6BIN)?;
    let record = db.ip_lookup("43.224.159.155".parse().unwrap())?;
    if let Record::LocationDb(rec) = record {
        assert!(rec.country.is_some());
        assert_eq!(rec.country.as_ref().unwrap().short_name, "IN");
        assert_eq!(rec.country.as_ref().unwrap().long_name, "India");
    }
    Ok(())
}

// Lookup an IPv4 in the IP2Location IPv4 BIN Database
fn ip_lookup_in_ipv4bin() -> Result<(), error::Error> {
    let db = DB::from_file(IPV4BIN)?;
    let record = db.ip_lookup("43.224.159.155".parse().unwrap())?;
    if let Record::LocationDb(rec) = record {
        assert!(rec.country.is_some());
        assert_eq!(rec.country.as_ref().unwrap().short_name, "IN");
        assert_eq!(rec.country.as_ref().unwrap().long_name, "India");
    }
    Ok(())
}

// Lookup an IP in the Proxy Database
fn ip_lookup_in_proxy_bin() -> Result<(), error::Error> {
    let db = DB::from_file(IP2PROXYBIN)?;
    let record = db.ip_lookup("1.1.1.1".parse().unwrap())?;
    if let Record::ProxyDb(rec) = record {
        assert!(rec.country.is_some());
    }
    Ok(())
}
```

### Executing the Example
```bash
cargo build --examples

# IP2Location Example
./target/debug/examples/lookup data/IP2LOCATION-LITE-DB1.IPV6.BIN 2a01:cb08:8d14::

# IP2Proxy Example
./target/debug/examples/lookup data/IP2PROXY-IP-COUNTRY.BIN 1.1.1.1
```

### License
This is free software, licensed under the MIT license.

### Ip2Location Databases:
- Lite free version: [Free](https://lite.ip2location.com/)
- Ip2Location / Ip2Proxy: [Commercial](https://ip2location.com/database/)

---
Sriram



