//! # A CHIP-8 cpu simulator and instruction coding/decoding library.
//!
//! `chip8` provides tools to enable working with
//! [CHIP-8 virtual machine](https://en.wikipedia.org/wiki/CHIP-8)
//! instructions, and a CHIP-8 virtual machine simulator. No GUI is provided,
//! `chip8` is intended to be used by another crate. For example, this crate could be used to:
//!
//!  * encode abstract instructions into machine code for a CHIP-8 assembler
//!  * decode binary machine code into abstract instruction for a CHIP-8 disassembler
//!  * simulate the execution of instructions in a CHIP-8 virtual machine for a GUI CHIP-8 emulator
//!
//! ```rust
//! use chip8::{Simulator, Simulate};
//! use chip8::instruction::Src::Register;
//! let mut chip8 = Simulator::default();
//! // A simple program:   LD v6, 25    LD v7, 17    ADD v6, v7
//! chip8.load_program( &[ 0x66, 0x19,  0x67, 0x11,  0x86, 0x74 ] ).unwrap();
//! // Execute the three instructions.
//! chip8.step_n(3).unwrap();
//! // Fetch the result from register v6.
//! let result = chip8.load(Register(6)).unwrap();
//! assert_eq!( result, 25+17 );
//! ```
//!
//! ## Instructions
//! An `instruction::Definition` is a combination of an `OperationKind` like *Add* or *Jump* and
//! a `Pattern` that specifies how the codword is structured.
//!
//! An `instruction::Set` contains one or more `Definition`. The `Set` can decode
//! machine codewords into `Operation`s and encode `Operation`s into codewords.
//!
//! ## Simulation
//! A `Chip8` represents the state of a CHIP-8 system, including CPU and peripherals.
//! It implements the `Execute` trait, so it is able to execute `Operation`s.
//!
//! A `Simulator` contains a `Chip8` and an `instruction::Set`, which allows it to decode
//! instructions and simulate their execution. The `Simulator` provides thread-safe mechanisms
//! for control of execution and inspection of machine state.


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
pub use simulator::{Simulator, Simulate};
