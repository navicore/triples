use lalrpop_util::lalrpop_mod;

pub mod data;
pub mod db;
pub mod db_api;
mod ttl_data;

lalrpop_mod!(pub ttl_prefix); // synthesized by LALRPOP
