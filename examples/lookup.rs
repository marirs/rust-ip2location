//! CLI example: look up an IP address in an IP2Location or IP2Proxy BIN database.
//!
//! Usage:
//!   cargo run --example lookup -- <path/to/db.bin> <ip_address>
//!
//! Example:
//!   cargo run --example lookup -- data/IP2LOCATION-LITE-DB1.IPV6.BIN 2a01:cb08:8d14::

use std::net::IpAddr;

use ip2location::DB;

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);

    let db_path = args.next().ok_or("Usage: lookup <db_path> <ip_address>")?;
    let db = DB::from_file(&db_path).map_err(|e| format!("Failed to open {db_path}: {e}"))?;

    let ip: IpAddr = args
        .next()
        .ok_or("Usage: lookup <db_path> <ip_address>")?
        .parse()
        .map_err(|e| format!("Invalid IP address: {e}"))?;

    db.print_db_info();
    println!();

    match db.ip_lookup(ip) {
        Ok(record) => println!("{:#?}", record),
        Err(e) => eprintln!("Lookup failed: {e}"),
    }

    Ok(())
}
