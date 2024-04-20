//! Tests for the "set on ___" instructions.
//!
//! Includes: seq, sne, slt, sltu, sle, sleu, sgt, sgtu, sge, sgeu.

use crate::emulation_core::architectures::AvailableDatapaths;

use super::*;

akin! {
    // These tests were created using a macro. These arrays can be seen as the "different"
    // parts of the tests. The `value1` and `value2` arrays are placed into registers and
    // are compared using different instructions.

    let &destination_register = [12, 13, 15, 17, 19, 21, 23, 25, 27, 29];

    let &instruction_name = [seq, sne,  slt,                  sltu,                 sle,  sleu, sgt,                  sgtu,                 sge,                  sgeu];
    //                                  -7                                                                            -1000
    let &true_value1 =      [56,  120,  18446744073709551609, 50,                   55,   120,  456,                  18446744073709550616, 12345,                98];
    //                                                        -40                               -456                                        -1234
    let &true_value2 =      [56,  22,   256,                  18446744073709551576, 55,   120,  18446744073709551160, 1000,                 18446744073709550382, 98];
    //                                                        -64                               -398                                        -5678
    let &false_value1 =     [34,  6969, 90,                   18446744073709551552, 4000, 90,   18446744073709551218, 70,                   18446744073709545938, 500];
    //                                                                                          -398
    let &false_value2 =     [99,  6969, 33,                   8,                    30,   20,   18446744073709551218, 130,                  5678,                 501];

    #[test]
    fn true_~*instruction_name() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions = String::from("*instruction_name r*destination_register, r5, r6");
        let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
        datapath.initialize_legacy(instruction_bits)?;

        datapath.registers.gpr[5] = *true_value1;
        datapath.registers.gpr[6] = *true_value2;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.registers.gpr[*destination_register], 1);
        Ok(())
    }

    #[test]
    fn false_~*instruction_name() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions = String::from("*instruction_name r*destination_register, r5, r6");
        let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
        datapath.initialize_legacy(instruction_bits)?;

        datapath.registers.gpr[5] = *false_value1;
        datapath.registers.gpr[6] = *false_value2;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.registers.gpr[*destination_register], 0);
        Ok(())
    }
}
