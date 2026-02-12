use crate::{error, Record, DB};
use std::io::Write;

const IPV4BIN: &str = "data/IP2LOCATION-LITE-DB1.BIN";
const IPV6BIN: &str = "data/IP2LOCATION-LITE-DB1.IPV6.BIN";
const IP2PROXYBIN: &str = "data/IP2PROXY-IP-COUNTRY.BIN";

#[test]
fn test_ipv4_lookup_in_ipv4bin() -> Result<(), error::Error> {
    let db = DB::from_file(IPV4BIN)?;
    let record = db.ip_lookup("43.224.159.155".parse().unwrap())?;
    let record = if let Record::LocationDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    assert!(record.is_some());
    let record = record.unwrap();
    assert!(record.country.is_some());
    assert_eq!(record.country.clone().unwrap().short_name, "IN");
    assert_eq!(record.country.unwrap().long_name, "India");
    Ok(())
}

#[test]
fn test_ipv4_lookup_in_ipv6bin() -> Result<(), error::Error> {
    let db = DB::from_file(IPV6BIN)?;
    let record = db.ip_lookup("43.224.159.155".parse().unwrap())?;
    let record = if let Record::LocationDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    assert!(record.is_some());
    let record = record.unwrap();
    assert!(record.country.is_some());
    assert_eq!(record.country.clone().unwrap().short_name, "IN");
    assert_eq!(record.country.unwrap().long_name, "India");
    Ok(())
}

#[test]
fn test_ipv6_lookup() -> Result<(), error::Error> {
    let db = DB::from_file(IPV6BIN)?;
    let record = db.ip_lookup("2a01:b600:8001::".parse().unwrap())?;
    let record = if let Record::LocationDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    assert!(record.is_some());
    let record = record.unwrap();
    assert!(record.country.is_some());
    assert_eq!(record.country.clone().unwrap().short_name, "IT");
    assert_eq!(record.country.unwrap().long_name, "Italy");
    Ok(())
}

#[test]
fn test_err_filenotfound_location() -> Result<(), error::Error> {
    let db = DB::from_file("nonexistant.bin");
    assert!(db.is_err());
    let result = &db.unwrap_err();
    let expected =
        &error::Error::IoError("Error opening DB file: No such file or directory".to_string());
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_ip_lookup_in_proxy_bin() -> Result<(), error::Error> {
    let db = DB::from_file(IP2PROXYBIN)?;
    let record = db.ip_lookup("1.1.1.1".parse().unwrap())?;
    let record = if let Record::ProxyDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    assert!(record.is_some());
    let record = record.unwrap();
    assert!(record.country.is_some());
    Ok(())
}

#[test]
fn test_ipv4_mapped_ipv6_lookup() -> Result<(), error::Error> {
    // ::ffff:43.224.159.155 should resolve the same as 43.224.159.155
    let db = DB::from_file(IPV6BIN)?;
    let record = db.ip_lookup("::ffff:43.224.159.155".parse().unwrap())?;
    let record = if let Record::LocationDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    assert!(record.is_some());
    let record = record.unwrap();
    assert!(record.country.is_some());
    assert_eq!(record.country.unwrap().short_name, "IN");
    Ok(())
}

#[test]
fn test_boundary_ip_zero() -> Result<(), error::Error> {
    let db = DB::from_file(IPV4BIN)?;
    // 0.0.0.0 may or may not be in the DB, but should not panic
    let result = db.ip_lookup("0.0.0.0".parse().unwrap());
    // Either Ok or RecordNotFound is acceptable
    match result {
        Ok(_) => {}
        Err(error::Error::RecordNotFound) => {}
        Err(e) => return Err(e),
    }
    Ok(())
}

#[test]
fn test_boundary_ip_max() -> Result<(), error::Error> {
    let db = DB::from_file(IPV4BIN)?;
    // 255.255.255.255 should not panic (u32::MAX edge case)
    let result = db.ip_lookup("255.255.255.255".parse().unwrap());
    match result {
        Ok(_) => {}
        Err(error::Error::RecordNotFound) => {}
        Err(e) => return Err(e),
    }
    Ok(())
}

#[test]
fn test_loopback_lookup() -> Result<(), error::Error> {
    let db = DB::from_file(IPV4BIN)?;
    let result = db.ip_lookup("127.0.0.1".parse().unwrap());
    match result {
        Ok(_) => {}
        Err(error::Error::RecordNotFound) => {}
        Err(e) => return Err(e),
    }
    Ok(())
}

#[test]
fn test_ipv6_loopback_lookup() -> Result<(), error::Error> {
    let db = DB::from_file(IPV6BIN)?;
    let result = db.ip_lookup("::1".parse().unwrap());
    match result {
        Ok(_) => {}
        Err(error::Error::RecordNotFound) => {}
        Err(e) => return Err(e),
    }
    Ok(())
}

#[test]
fn test_ipv6_unspecified_lookup() -> Result<(), error::Error> {
    let db = DB::from_file(IPV6BIN)?;
    let result = db.ip_lookup("::".parse().unwrap());
    match result {
        Ok(_) => {}
        Err(error::Error::RecordNotFound) => {}
        Err(e) => return Err(e),
    }
    Ok(())
}

#[test]
fn test_proxy_ipv4_boundary() -> Result<(), error::Error> {
    let db = DB::from_file(IP2PROXYBIN)?;
    let result = db.ip_lookup("255.255.255.255".parse().unwrap());
    match result {
        Ok(_) => {}
        Err(error::Error::RecordNotFound) => {}
        Err(e) => return Err(e),
    }
    Ok(())
}

#[test]
fn test_to_json_location() -> Result<(), error::Error> {
    let db = DB::from_file(IPV4BIN)?;
    let record = db.ip_lookup("43.224.159.155".parse().unwrap())?;
    if let Record::LocationDb(rec) = record {
        let json = rec.to_json();
        assert!(json.contains("43.224.159.155"));
        assert!(json.contains("IN"));
        assert!(json.contains("India"));
    } else {
        panic!("Expected LocationDb record");
    }
    Ok(())
}

#[test]
fn test_to_json_proxy() -> Result<(), error::Error> {
    let db = DB::from_file(IP2PROXYBIN)?;
    let record = db.ip_lookup("1.1.1.1".parse().unwrap())?;
    if let Record::ProxyDb(rec) = record {
        let json = rec.to_json();
        assert!(json.contains("1.1.1.1"));
    } else {
        panic!("Expected ProxyDb record");
    }
    Ok(())
}

#[test]
fn test_truncated_db_file() {
    // Create a tiny file that's too small to be a valid DB
    let path = "data/test_truncated.bin";
    {
        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(&[0u8; 10]).unwrap();
    }
    let result = DB::from_file(path);
    assert!(result.is_err());
    // Clean up
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_corrupt_db_file() {
    // Create a file with enough bytes for a header but garbage content
    let path = "data/test_corrupt.bin";
    {
        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(&[0xFF; 64]).unwrap();
    }
    let result = DB::from_file(path);
    assert!(result.is_err());
    // Clean up
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_db_info_does_not_panic() -> Result<(), error::Error> {
    let db = DB::from_file(IPV4BIN)?;
    db.print_db_info(); // Should not panic
    let db = DB::from_file(IP2PROXYBIN)?;
    db.print_db_info(); // Should not panic
    Ok(())
}

#[test]
fn test_multiple_ipv4_lookups() -> Result<(), error::Error> {
    let db = DB::from_file(IPV4BIN)?;
    let ips = ["8.8.8.8", "1.1.1.1", "43.224.159.155", "203.0.113.1"];
    for ip_str in &ips {
        let result = db.ip_lookup(ip_str.parse().unwrap());
        match result {
            Ok(Record::LocationDb(rec)) => {
                assert!(rec.country.is_some());
            }
            Err(error::Error::RecordNotFound) => {}
            other => panic!("Unexpected result for {}: {:?}", ip_str, other),
        }
    }
    Ok(())
}

#[test]
fn test_multiple_ipv6_lookups() -> Result<(), error::Error> {
    let db = DB::from_file(IPV6BIN)?;
    let ips = ["2a01:b600:8001::", "2001:4860:4860::8888", "2606:4700:4700::1111"];
    for ip_str in &ips {
        let result = db.ip_lookup(ip_str.parse().unwrap());
        match result {
            Ok(Record::LocationDb(rec)) => {
                assert!(rec.country.is_some());
            }
            Err(error::Error::RecordNotFound) => {}
            other => panic!("Unexpected result for {}: {:?}", ip_str, other),
        }
    }
    Ok(())
}
