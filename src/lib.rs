mod common;
pub use common::{Record, DB};

pub mod error;

mod ip2location;
pub use self::ip2location::{db::LocationDB, record::LocationRecord};

mod ip2proxy;
pub use self::ip2proxy::{db::ProxyDB, record::ProxyRecord};

#[cfg(test)]
mod tests;
