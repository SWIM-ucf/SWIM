//! Tests for the double arithmetic instructions: dadd, dsub, dmul, ddiv, daddu, dsubu, dmulu, ddivu.

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
        datapath.initialize_legacy(instruction_bits)?;

        datapath.registers.gpr[16] = *value1;
        datapath.registers.gpr[17] = *value2;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.registers.gpr[*result_register], *expected_result);
        Ok(())
    }
}

akin! {
    let &instruction_name = [daddu,                 dsubu,                 dmulu,                 ddivu];
    let &instruction =      ["daddu r10, r25, r26", "dsubu r11, r25, r26", "dmulu r12, r25, r26", "ddivu r13, r25, r26"];
    let &value1 =           [12519072089974610290,  841351681,             804468187,             6100876364229782140];
    let &value2 =           [532,                   1651181918911,         10630297190,           1220];
    let &result_register =  [10,                    11,                    12,                    13];
    //                                              -1650340567230
    let &expected_result =  [12519072089974610822,  18446742423368984386,  8551735907710494530,   5000718331335887];

    #[test]
    fn basic_~*instruction_name() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions = String::from(*instruction);
        let (_, instruction_bits) = parser(instructions);
        datapath.initialize_legacy(instruction_bits)?;

        datapath.registers.gpr[25] = *value1;
        datapath.registers.gpr[26] = *value2;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.registers.gpr[*result_register], *expected_result);
        Ok(())
    }
}
