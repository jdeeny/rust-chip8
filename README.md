# rust-chip8

This crate contains tools to enable working with [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
virtual machine instructions and a CHIP-8 virtual machine simulator. It is
intended to be used as the core of CHIP-8 emulators, CHIP-8 assemblers, CHIP-8
disassmblers, and other programs that need to deal with CHIP-8 instructions.

Instruction definitions for the original CHIP-8 instruction set and several instruction set
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
These `Definition`s can be combined into an `instruction::Set` that is able to
*decode* codewords into `Operation`s and *encode* `Operation`s into codewords.

## Simulation
The state of a CHIP-8 system, including CPU and peripherals, is represented by a `Chip8`.
It is able to execute instructions by providing an implementation of the `Execute` trait.

A `Simulator` contains a `Chip8` and an `instruction::Set`, which allows it to simulate
a CHIP-8 machine fetching codewords from memory, decoding the codewords, and executing the
resulting instructions. The `Simulator` provides a mechanism for thread-safe control
of the machine and access to state information.
