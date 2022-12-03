mod common;
pub use common::DB;

pub mod error;

mod ip2location;
pub use ip2location::db::LocationDB;

mod ip2proxy;
pub use ip2proxy::db::ProxyDB;

#[cfg(test)]
mod tests;
