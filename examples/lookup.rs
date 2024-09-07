use std::net::IpAddr;

use ip2location::DB;

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut db = match DB::from_file(
        &*args
            .next()
            .ok_or("First argument is the path to db")?,
    ) {
        Ok(db) => db,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let ip: IpAddr = args
        .next()
        .ok_or("Second argument must be the IP address, like 128.101.101.101")?
        .parse()
        .unwrap();

    // print the db information
    db.print_db_info();
    println!();

    // print the IP information
    match db.ip_lookup(ip) {
        Ok(record) => println!("{:#?}", record),
        Err(e) => println!("{:?}", e),
    };

    Ok(())
}
