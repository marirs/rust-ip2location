//! IP2Location / IP2Proxy BIN database reader.
//!
//! Provides zero-copy, memory-mapped lookups against
//! [IP2Location](https://www.ip2location.com/) and
//! [IP2Proxy](https://www.ip2proxy.com/) BIN databases.
//!
//! # Quick start
//!
//! ```rust
//! use ip2location::{DB, Record};
//!
//! let db = DB::from_file("data/IP2LOCATION-LITE-DB1.BIN").unwrap();
//! let record = db.ip_lookup("8.8.8.8".parse().unwrap()).unwrap();
//! println!("{:#?}", record);
//! ```
//!
//! The database file is memory-mapped at open time and string fields in
//! returned records borrow directly from the mapped region (`Cow::Borrowed`),
//! avoiding heap allocation on the hot path.

mod common;
pub use common::{Record, DB};

pub mod error;

mod ip2location;
pub use self::ip2location::{db::LocationDB, record::LocationRecord};

mod ip2proxy;
pub use self::ip2proxy::{
    db::ProxyDB,
    record::{Proxy, ProxyRecord},
};

#[cfg(test)]
mod tests;
