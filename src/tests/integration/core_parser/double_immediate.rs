//! Tests for the double immediate instructions: dahi, dati, daddi, dsubi, dmuli, ddivi, daddiu, dsubiu, dmuliu, ddiviu.

use super::*;

#[test]
fn basic_dahi() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("dahi r3, 123");
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[3] = 0;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    // 123 should fill in bits 32-47.
    // 0000000000000000 0000000001111011 0000000000000000 0000000000000000
    //                  ^^^^^^^^^^^^^^^^
    assert_eq!(datapath.registers.gpr[3], 0x0000_007B_0000_0000);
    Ok(())
}

#[test]
fn dahi_sign_extend() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("dahi r5, 43158");
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[5] = 0;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    // 43158 should fill in bits 32-47. Since the bit 47 is a 1, this is sign-extended with 1's.
    // 1111111111111111 1010100010010110 0000000000000000 0000000000000000
    //                  ^^^^^^^^^^^^^^^^
    assert_eq!(datapath.registers.gpr[5], 0xFFFF_A896_0000_0000);
    Ok(())
}

#[test]
fn basic_dati() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("dati r10, 4321");
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[10] = 0;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    // 4321 should fill in bits 48-63.
    // 0001000011100001 0000000000000000 0000000000000000 0000000000000000
    // ^^^^^^^^^^^^^^^^
    assert_eq!(datapath.registers.gpr[10], 0x10E1_0000_0000_0000);
    Ok(())
}

akin! {
    let &instruction_name = [daddi,                   dsubi,                   dmuli,                   ddivi];
    let &instruction =      ["daddi r31, r20, 15623", "dsubi r30, r20, 25561", "dmuli r29, r20, 20000", "ddivi r28, r20, 351"];
    let &rs_value =         [1825494919615,           6481676184919,           65165189491,             5259435330591];
    let &result_register =  [31,                      30,                      29,                      28];
    let &expected_result =  [1825494935238,           6481676159358,           1303303789820000,        14984146241];

    #[test]
    fn basic_~*instruction_name() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions = String::from(*instruction);
        let (_, instruction_bits) = parser(instructions);
        datapath.initialize(instruction_bits)?;

        datapath.registers.gpr[20] = *rs_value;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.registers.gpr[*result_register], *expected_result);
        Ok(())
    }
}

akin! {
    let &instruction_name = [daddiu,                  dsubiu,                  dmuliu,                ddiviu];
    let &instruction =      ["daddiu r27, r20, 6543", "dsubiu r26, r20, 4130", "dmuliu r25, r20, 30", "ddiviu r24, r20, 84"];
    let &rs_value =         [541984981984,            981987489179,            86919841687945,        63953229593232];
    let &result_register =  [27,                      26,                      25,                    24];
    let &expected_result =  [541984988527,            981987485049,            2607595250638350,      761347971348];

    #[test]
    fn basic_~*instruction_name() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions = String::from(*instruction);
        let (_, instruction_bits) = parser(instructions);
        datapath.initialize(instruction_bits)?;

        datapath.registers.gpr[20] = *rs_value;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.registers.gpr[*result_register], *expected_result);
        Ok(())
    }
}
