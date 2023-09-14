use lalrpop_util::lalrpop_mod;

pub mod data;
pub mod db;
pub mod db_api;

lalrpop_mod!(pub ttl_prefix); // synthesized by LALRPOP
