//! A chip8 cpu simulator and instruction coding/decoding library.
//!
//! This crate is able to define a configuration for a Chip8 virtual machine, allowing
//! customization of parameters such as RAM size, system fonts, quirks, and supported instruction
//! sets. Several predefined `Config`s are provided in the presets module.
//!
//! An `Set` can be created with a particular `Config`. It will contain a set of instruction
//! definitions that will allow it to decode a codeword into an abstact `Instruction`. An `Instruction`
//! can likewise be encoded into a `Codeword``.
//!
//! A `Chip8` can be created with a particular `Config`. It will contain the state of a Chip8
//! virtual machine. It implements the `Executor` trait, which allows it to execute `Microprogram`s.
//!
//! A `Simulator` provides a thread-safe interface to a Chip8 simulator. It contains an
//! `Set` and a `Chip8`. It is able to load programs and execute instructions.

#![feature(inclusive_range_syntax)]
#![feature(plugin)]
#![plugin(drawbytes)]

// enable errors for some additional lints
#![deny(trivial_casts, trivial_numeric_casts, unsafe_code,
        unused_qualifications)]

#![cfg_attr(feature = "lints", warn(missing_docs))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(clippy))]
#![cfg_attr(feature = "lints", deny(if_not_else, invalid_upcast_comparisons, items_after_statements,
                                    mem_forget, mut_mut, mutex_integer, non_ascii_literal,
                                    nonminimal_bool, shadow_reuse, shadow_same, shadow_unrelated,
                                    similar_names, single_match_else, string_add, string_add_assign,
                                    unicode_not_nfc, wrong_pub_self_convention ))]
#![cfg_attr(feature = "lints", warn(option_unwrap_used, result_unwrap_used, missing_debug_implementations,
                                    unused_import_braces,missing_copy_implementations))]
//#![cfg_attr(feature = "lints", cast_possible_truncation, cast_possible_wrap, cast_sign_loss))]

#![allow(unused_imports, unused_variables, unused_mut)]

extern crate rand;
extern crate strfmt;

pub mod config;
pub mod fonts;
pub mod instruction;
//pub mod simulator;
mod state;
mod types;

pub use config::Config;
pub use types::*;
pub use state::Chip8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8_system() {
        let conf = config::presets::COSMAC_VIP_STOCK;
        let chip8 = Chip8::new(&conf);
        let mut isa = instruction::Set::new(&conf);
        let d = instruction::Decoder::new(&conf, &isa);
        let e = instruction::Encoder::new(&conf, &isa);

        assert!(true)
    }
}
