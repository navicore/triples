use lalrpop_util::lalrpop_mod;

pub mod csv_file;
pub mod data;
pub mod db;
pub mod db_api;
pub mod sparql_data;
pub mod ttl_file;
pub mod ttl_stream;
pub mod turtle_stream;

lalrpop_mod!(#[allow(clippy::complexity, clippy::pedantic, clippy::restriction, clippy::nursery)] pub ttl);

lalrpop_mod!(#[allow(clippy::complexity, clippy::pedantic, clippy::restriction, clippy::nursery)] pub sparql);

lalrpop_mod!(#[allow(clippy::complexity, clippy::pedantic, clippy::restriction, clippy::nursery)] pub turtle);
