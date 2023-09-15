use lalrpop_util::lalrpop_mod;

pub mod data;
pub mod db;
pub mod db_api;
pub mod ttl_data;

lalrpop_mod!(#[allow(clippy::complexity, clippy::pedantic, clippy::restriction, clippy::nursery)] pub ttl);
