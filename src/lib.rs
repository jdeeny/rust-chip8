//! A chip8 cpu simulator and instruction coding/decoding library.
//!
//! This crate is able to define a configuration for a Chip8 virtual machine, allowing
//! customization of parameters such as RAM size, system fonts, quirks, and supported instruction
//! sets. Several predefined `Config`s are provided in the presets module.
//!
//! An `InstructionSet` can be created with a particular `Config`. It will contain a set of instruction
//! definitions that will allow it to decode a codeword into an abstact `Instruction`. An `Instruction`
//! can likewise be encoded into a `Codeword``.
//!
//! A `Chip8` can be created with a particular `Config`. It will contain the state of a Chip8
//! virtual machine. It implements the `Executor` trait, which allows it to execute `Operation`s.
//!
//! A `Simulator` provides a thread-safe interface to a Chip8 simulator. It contains an
//! `InstructionSet` and a `Chip8`. It is able to load programs and execute instructions.

#![feature(inclusive_range_syntax)]
#![feature(plugin)]
#![plugin(drawbytes)]

// enable errors for some additional lints
#![deny(missing_copy_implementations,
        trivial_casts, trivial_numeric_casts, unsafe_code, unused_import_braces,
        unused_qualifications)]

#![cfg_attr(feature = "lints", warn(missing_docs))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(clippy))]
#![cfg_attr(feature = "lints", deny(if_not_else, invalid_upcast_comparisons, items_after_statements,
                                    mem_forget, mut_mut, mutex_integer, non_ascii_literal,
                                    nonminimal_bool, shadow_reuse, shadow_same, shadow_unrelated,
                                    similar_names, single_match_else, string_add, string_add_assign,
                                    unicode_not_nfc, wrong_pub_self_convention ))]
#![cfg_attr(feature = "lints", warn(option_unwrap_used, result_unwrap_used, missing_debug_implementations))]
//#![cfg_attr(feature = "lints", cast_possible_truncation, cast_possible_wrap, cast_sign_loss))]

extern crate rand;
extern crate strfmt;

pub mod instructions;
pub mod fonts;
pub mod chip8;
mod config;
//pub mod simulator;

mod types;

pub use chip8::Chip8;
pub use config::{Config, presets};
