use lalrpop_util::lalrpop_mod;

pub mod data;
pub mod db;
pub mod db_api;
mod ttl_data;

lalrpop_mod!(#[allow(clippy::pedantic, clippy::restriction, clippy::nursery)] pub ttl_prefix);

lalrpop_mod!(#[allow(clippy::pedantic, clippy::restriction, clippy::nursery)] pub ttl_body);
