//! A chip8 cpu simulator and instruction coding/decoding library.
//!
//!

#![feature(inclusive_range_syntax)]
#![cfg_attr(feature = "lints", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]

// additional lints
#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts, unsafe_code, unused_import_braces,
        unused_qualifications)]
// clippy lints
#![cfg_attr(feature = "lints", deny(clippy))]
#![cfg_attr(feature = "lints", deny(if_not_else, invalid_upcast_comparisons, items_after_statements,
                                    mem_forget, mut_mut, mutex_integer, non_ascii_literal,
                                    nonminimal_bool, shadow_reuse, shadow_same, shadow_unrelated,
                                    similar_names, single_match_else, string_add, string_add_assign,
                                    unicode_not_nfc, wrong_pub_self_convention ))]
// very pedantic clippy lints
//#![cfg_attr(feature = "lints", warn(option_unwrap_used, result_unwrap_used, cast_possible_truncation, cast_possible_wrap, cast_sign_loss))]
// also warn about missing documentation
#![cfg_attr(feature = "lints", warn(missing_docs))]

extern crate rand;
extern crate strfmt;

mod config;
mod fonts;


pub mod instruction;
pub mod simulator;

pub use config::Config;
