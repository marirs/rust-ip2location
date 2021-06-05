use crate::*;

const IPV4BIN: &str = "data/IP2LOCATION-LITE-DB1.BIN";
const IPV6BIN: &str = "data/IP2LOCATION-LITE-DB1.IPV6.BIN";

#[test]
fn test_ipv4_lookup_in_ipv4bin() -> Result<(), error::Error> {
    let mut db = DB::from_file(IPV4BIN)?;
    let record = db.ip_lookup("43.224.159.155")?;
    assert!(!record.country.is_none());
    assert_eq!(record.country.clone().unwrap().short_name, "IN");
    assert_eq!(record.country.unwrap().long_name, "India");
    Ok(())
}

#[test]
fn test_ipv4_lookup_in_ipv4bin_using_mmap() -> Result<(), error::Error> {
    let mut db = DB::from_file_mmap(IPV4BIN)?;
    let record = db.ip_lookup("43.224.159.155")?;
    assert!(!record.country.is_none());
    assert_eq!(record.country.clone().unwrap().short_name, "IN");
    assert_eq!(record.country.unwrap().long_name, "India");
    Ok(())
}

#[test]
fn test_ipv4_lookup_in_ipv6bin() -> Result<(), error::Error> {
    let mut db = DB::from_file(IPV6BIN)?;
    let record = db.ip_lookup("43.224.159.155")?;
    assert!(!record.country.is_none());
    assert_eq!(record.country.clone().unwrap().short_name, "IN");
    assert_eq!(record.country.unwrap().long_name, "India");
    Ok(())
}

#[test]
fn test_ipv4_lookup_in_ipv6bin_using_mmap() -> Result<(), error::Error> {
    let mut db = DB::from_file_mmap(IPV6BIN)?;
    let record = db.ip_lookup("43.224.159.155")?;
    assert!(!record.country.is_none());
    assert_eq!(record.country.clone().unwrap().short_name, "IN");
    assert_eq!(record.country.unwrap().long_name, "India");
    Ok(())
}

#[test]
fn test_ipv6_lookup() -> Result<(), error::Error> {
    let mut db = DB::from_file(IPV6BIN)?;
    let record = db.ip_lookup("2a01:b600:8001::")?;
    assert!(!record.country.is_none());
    assert_eq!(record.country.clone().unwrap().short_name, "IT");
    assert_eq!(record.country.unwrap().long_name, "Italy");
    Ok(())
}

#[test]
fn test_ipv6_lookup_using_mmap() -> Result<(), error::Error> {
    let mut db = DB::from_file_mmap(IPV6BIN).unwrap();
    let record = db.ip_lookup("2a01:cb08:8d14::")?;
    assert!(!record.country.is_none());
    assert_eq!(record.country.clone().unwrap().short_name, "FR");
    assert_eq!(record.country.unwrap().long_name, "France");
    Ok(())
}

#[test]
fn test_err_filenotfound() -> Result<(), error::Error> {
    let db = DB::from_file("nonexistant.bin");
    assert!(db.is_err());
    let result = &db.unwrap_err();
    let expected = &error::Error::IoError(
        "Error opening DB file: No such file or directory (os error 2)".to_string(),
    );
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_err_invalidipaddress() -> Result<(), error::Error> {
    let mut db = DB::from_file(IPV4BIN)?;
    let record = db.ip_lookup("invalid");
    let expected = Err(error::Error::InvalidIP("ip address is invalid".to_string()));
    assert_eq!(record, expected);
    Ok(())
}
