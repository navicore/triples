use lalrpop_util::lalrpop_mod;

pub mod data;
pub mod db;
pub mod db_api;
mod ttl_data;

// the absurd list of lints is because clippy::all has no effect
lalrpop_mod!(#[allow(
        clippy::missing_errors_doc,
        clippy::missing_const_for_fn,
        clippy::unwrap_used,
        clippy::no_effect_underscore_binding,
        clippy::redundant_pub_crate,
        clippy::uninlined_format_args,
        clippy::cast_sign_loss,
        clippy::option_if_let_else,
        clippy::use_self,
        clippy::must_use_candidate,
        clippy::needless_pass_by_value,
        clippy::unnested_or_patterns,
        clippy::all
        )] pub ttl_prefix);
