# rust-chip8: A CHIP-8 CPU simulator and instruction coding/decoding library.

## Introduction
This crate contains tools to enable working with [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
virtual machine instructions and a CHIP-8 virtual machine simulator.

Instruction defintions for the original CHIP-8 instruction set and several instruction set
extensions are provided. Decoding and encoding functions are provided to enable converting
from binary codewords to an abstract instruction representation and vice versa.

A CHIP-8 virtual machine simulator system is provided to allow the creation of CHIP-8 emulators
or other tools that need to execute CHIP-8 instructions. The simulator provides a thread-safe
mechanism for simulator control and access to system state.

A flexible configuration system allows customization of the CHIP-8 virtual machine, including
available instructions, hardware configuration, and execution quirks of the CHIP-8 virtual
machine. Several preset configurations corresponding to historical CHIP-8 machines, such as
the COSMAC VIP, are provided.

## Configuration

A `Config` is used to store the configuration of a specific CHIP-8 system. A `Config` is
used when instantiating machine state or an instruction set.

## Instructions
A set of `Operation`s are supported by this library. These operations correspond
with instructions from the CHIP-8 instruction set, but are typically more flexible to allow
re-use.

A `Pattern` defines how the operands required by an `Operation` will be defined by the
codeword.

An `instruction::Definition` is a combination of an `Operation` and a `Pattern`.
These `Definition`s can be combined into an `instruction::Set` that is able to *decode* codewords
and return `instruction::Instruction`s and *encode* `instruction::Instruction`s into codewords.

`Definition`s and `Instruction`s are very similar, but a key difference sets them apart - a
`Definition` only stores the *kind* of operands that will be used and the `Instruction` stores
the specific operands that will be used. When a codeword is decoded, information about the
operands is extracted based on the pattern in the definition. For example, a generic 'Register'
operand in a `Definition` could be specified as 'the v6 register' when the Instruction is
created.

## Simulation
The state of a CHIP-8 system, including CPU and peripherals, is represented by a `Chip8`.
It is able to execute instructions by providing an implementation of the `Execute` trait.

A `Simulator` contains a `Chip8` and an `instruction::Set`, which allows it to simulate
a CHIP-8 machine fetching codewords from memory, decoding the codewords, and executing the
resulting instructions. The `Simulator` provides a mechanism for thread-safe control
of the machine and access to state information.
