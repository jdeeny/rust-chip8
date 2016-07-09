/// A table of all the chip8 instructions. TODO: Move to new file
pub struct Table {
    table: Vec<Definition>,
}
impl Table {
    /// Returns a new instruction table.
    #[allow(unused_variables)]
    pub fn new(config: Config) -> Table {

        let mut table: Vec<Definition> = Vec::new();
        for d in isa_chip8 {
            //table.push(*d);
        }

        Table { table: table }
    }
}

impl Table {
    /// Decode a codeword by finding a match in the table.
    pub fn decode(&self, codeword: Codeword) -> Instruction {
/*        for def in &self.table {
            if def.is_match(codeword) {
                return Instruction::new(def.clone(), codeword);
            }
        }*/
        panic!("Unknown Instruction")
    }

    /// Encode an `Instruction` into a codeword.
    #[allow(unused_variables)]
    pub fn encode(&self, inst: Instruction) -> Codeword {
        0
        /*for def in &self.table {
            if def.operation == inst.operation {

            }
        }*/
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Table {{}}")
    }
}
