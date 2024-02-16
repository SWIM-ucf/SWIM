//! Tests for the floating-point arithmetic instructions: add.s, add.d, sub.s, sub.d, mul.s, mul.d, div.s, div.d

use super::*;

akin! {
    let &instruction_name = [add_s,                    sub_s,                    mul_s,                    div_s];
    let &instruction =      ["add.s $f20, $f15, $f16", "sub.s $f21, $f15, $f16", "mul.s $f22, $f15, $f16", "div.s $f23, $f15, $f16"];
    //                       3.0                       120.125                   5.5                       50.125
    let &value1 =           [1077936128,               1123041280,               1085276160,               1112047616];
    //                       150.5                     55.75                     6.5                       2.0
    let &value2 =           [1125548032,               1113522176,               1087373312,               1073741824];
    let &result_register =  [20,                       21,                       22,                       23];
    //                       153.5                     64.375                    35.75                     25.0625
    let &expected_result =  [1125744640,               1115734016,               1108279296,               1103659008];

    #[test]
    fn basic_~*instruction_name() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions = String::from(*instruction);
        let (_, instruction_bits) = parser(instructions);
        datapath.initialize_legacy(instruction_bits)?;

        datapath.coprocessor.fpr[15] = *value1;
        datapath.coprocessor.fpr[16] = *value2;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.coprocessor.fpr[*result_register], *expected_result);
        Ok(())
    }
}

akin! {
    let &instruction_name = [add_d,                    sub_d,                    mul_d,                    div_d];
    let &instruction =      ["add.d $f20, $f15, $f16", "sub.d $f21, $f15, $f16", "mul.d $f22, $f15, $f16", "div.d $f23, $f15, $f16"];
    //                       3.0                       120.125                   5.5                       50.125
    let &value1 =           [4613937818241073152,      4638153462331211776,      4617878467915022336,      4632251283913310208];
    //                       150.5                     55.75                     6.5                       2.0
    let &value2 =           [4639499264563609600,      4633042932285308928,      4619004367821864960,      4611686018427387904];
    let &result_register =  [20,                       21,                       22,                       23];
    //                       153.5                     64.375                    35.75                     25.0625
    let &expected_result =  [4639604817679876096,      4634230404843307008,      4630228182518202368,      4627747684285939712];

    #[test]
    fn basic_~*instruction_name() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions = String::from(*instruction);
        let (_, instruction_bits) = parser(instructions);
        datapath.initialize_legacy(instruction_bits)?;

        datapath.coprocessor.fpr[15] = *value1;
        datapath.coprocessor.fpr[16] = *value2;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.coprocessor.fpr[*result_register], *expected_result);
        Ok(())
    }
}
