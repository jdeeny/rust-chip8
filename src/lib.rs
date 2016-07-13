//! # A CHIP-8 cpu simulator and instruction coding/decoding library.
//!
//! ## Introduction
//! This crate contains tools to enable working with [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
//! virtual machine instructions and a CHIP-8 virtual machine simulator.
//!
//! Instruction defintions for the original CHIP-8 instruction set and several instruction set
//! extensions are provided. Decoding and encoding functions are provided to enable converting
//! from binary codewords to an abstract instruction representation and vice versa.
//!
//! A CHIP-8 virtual machine simulator system is provided to allow the creation of CHIP-8 emulators
//! or other tools that need to execute CHIP-8 instructions. The simulator provides a thread-safe
//! mechanism for simulator control and access to system state.
//!
//! A flexible configuration system allows customization of the CHIP-8 virtual machine, including
//! available instructions, hardware configuration, and execution quirks of the CHIP-8 virtual
//! machine. Several preset configurations corresponding to historical CHIP-8 machines, such as
//! the COSMAC VIP, are provided.
//!
//! ## Configuration
//!
//! A `Config` is used to store the configuration of a specific CHIP-8 system. A `Config` is
//! used when instantiating machine state or an instruction set.
//!
//! ## Instructions
//! A set of `Operation`s are supported by this library. These operations correspond
//! with instructions from the CHIP-8 instruction set, but are typically more flexible to allow
//! re-use.
//!
//! A `Pattern` defines how the operands required by an `Operation` will be defined by the
//! codeword.
//!
//! An `instruction::Definition` is a combination of an `Operation` and a `Pattern`.
//! These `Definition`s can be combined into an `instruction::Set` that is able to *decode* codewords
//! and return `instruction::Instruction`s and *encode* `instruction::Instruction`s into codewords.
//!
//! `Definition`s and `Instruction`s are very similar, but a key difference sets them apart - a
//! `Definition` only stores the *kind* of operands that will be used and the `Instruction` stores
//! the specific operands that will be used. When a codeword is decoded, information about the
//! operands is extracted based on the pattern in the definition. For example, a generic 'Register'
//! operand in a `Definition` could be specified as 'the v6 register' when the Instruction is
//! created.
//!
//! ## Simulation
//! The state of a CHIP-8 system, including CPU and peripherals, is represented by a `Chip8`.
//! It is able to execute instructions by providing an implementation of the `Execute` trait.
//!
//! A `Simulator` contains a `Chip8` and an `instruction::Set`, which allows it to simulate
//! a CHIP-8 machine fetching codewords from memory, decoding the codewords, and executing the
//! resulting instructions. The `Simulator` provides a mechanism for thread-safe control
//! of the machine and access to state information.


#![feature(inclusive_range_syntax, fnbox, plugin)]
#![feature(question_mark)]
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
#![cfg_attr(feature = "lints", allow(doc_markdown))]


//#![cfg_attr(feature = "lints", cast_possible_truncation, cast_possible_wrap, cast_sign_loss))]

#![allow(unused_imports, unused_variables, unused_mut)]

extern crate rand;
extern crate strfmt;

pub mod config;
pub mod fonts;
pub mod instruction;
mod simulator;
mod state;
mod types;

pub use config::Config;
pub use types::*;
pub use state::Chip8;
pub use simulator::Simulator;

#[cfg(test)]
mod tests {
    use super::*;

/*    #[test]
    fn test_chip8_system() {
        let conf = config::presets::COSMAC_VIP_STOCK;
        let chip8 = Chip8::new(&conf, None);
        let mut isa = instruction::Set::new(&conf);
        let d = instruction::Decoder::new(&conf, &isa);
        let e = instruction::Encoder::new(&conf, &isa);

        assert!(true)
    }*/
}
