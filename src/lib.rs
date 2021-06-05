mod consts;
mod db;
pub use self::db::DB;
pub mod error;
pub mod record;

#[cfg(test)]
mod tests;
