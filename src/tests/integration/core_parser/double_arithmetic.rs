//! Tests for the double arithmetic instructions: dadd, dsub, dmul, ddiv.

use super::*;

akin! {
    let &instruction_name = [dadd,                 dsub,                 dmul,                 ddiv];
    let &instruction =      ["dadd r15, r16, r17", "dsub r19, r16, r17", "dmul r24, r16, r17", "ddiv r14, r16, r17"];
    let &value1 =           [187650270761524,      55981,                1353171,              240496068448256];
    let &value2 =           [1000,                 78198451644,          5432,                 5];
    let &result_register =  [15,                   19,                   24,                   14];
    //                                             -78198395663
    let &expected_result =  [187650270762524,      18446743995511155953, 7350424872,           48099213689651];

    #[test]
    fn basic_~*instruction_name() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions = String::from(*instruction);
        let (_, instruction_bits) = parser(instructions);
        datapath.initialize(instruction_bits)?;

        datapath.registers.gpr[16] = *value1;
        datapath.registers.gpr[17] = *value2;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.registers.gpr[*result_register], *expected_result);
        Ok(())
    }
}
