use crate::{error, Record, DB};

const IPV4BIN: &str = "data/IP2LOCATION-LITE-DB1.BIN";
const IPV6BIN: &str = "data/IP2LOCATION-LITE-DB1.IPV6.BIN";
const IP2PROXYBIN: &str = "data/IP2PROXY-IP-COUNTRY.BIN";

#[test]
fn test_ipv4_lookup_in_ipv4bin() -> Result<(), error::Error> {
    let mut db = DB::from_file(IPV4BIN)?;
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
    let mut db = DB::from_file(IPV6BIN)?;
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
    let mut db = DB::from_file(IPV6BIN)?;
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
    let mut db = DB::from_file(IP2PROXYBIN)?;
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
