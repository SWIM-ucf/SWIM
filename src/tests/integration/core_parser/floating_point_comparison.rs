//! Tests for the floating-point comparison instructions: c.eq.s, c.eq.d, c.lt.s, c.lt.d, c.le.s, c.le.d, c.ngt.s, c.ngt.d, c.nge.s, c.nge.d

use super::*;

akin! {
    let &instruction_name = [c_eq_s,              c_lt_s,              c_le_s,              c_ngt_s,              c_nge_s];
    let &instruction =      ["c.eq.s $f15, $f16", "c.lt.s $f15, $f16", "c.le.s $f15, $f16", "c.ngt.s $f15, $f16", "c.nge.s $f15, $f16"];
    //                       3.0                  120.125              5.5                  50.125                69.0
    let &value1 =           [1077936128,          1123041280,          1085276160,          1112047616,           1116340224];
    //                       150.5                55.75                6.5                  2.0                   69.5
    let &value2 =           [1125548032,          1113522176,          1087373312,          1073741824,           1116405760];
    let &expected_result =  [0,                   0,                   1,                   0,                    1];

    #[test]
    fn basic_~*instruction_name() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions = String::from(*instruction);
        let (_, instruction_bits) = parser(instructions);
        datapath.initialize(instruction_bits)?;

        datapath.coprocessor.fpr[15] = *value1;
        datapath.coprocessor.fpr[16] = *value2;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.coprocessor.condition_code, *expected_result);
        Ok(())
    }
}

akin! {
    let &instruction_name = [c_eq_d,              c_lt_d,              c_le_d,              c_ngt_d,              c_nge_d];
    let &instruction =      ["c.eq.d $f15, $f16", "c.lt.d $f15, $f16", "c.le.d $f15, $f16", "c.ngt.d $f15, $f16", "c.nge.d $f15, $f16"];
    //                       3.0                  120.125              5.5                  50.125                69.0
    let &value1 =           [4613937818241073152, 4638153462331211776, 4617878467915022336, 4632251283913310208,  4634555860285128704];
    //                       150.5                55.75                6.5                  2.0                   69.5
    let &value2 =           [4639499264563609600, 4633042932285308928, 4619004367821864960, 4611686018427387904,  4634591044657217536];
    let &expected_result =  [0,                   0,                   1,                   0,                    1];

    #[test]
    fn basic_~*instruction_name() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions = String::from(*instruction);
        let (_, instruction_bits) = parser(instructions);
        datapath.initialize(instruction_bits)?;

        datapath.coprocessor.fpr[15] = *value1;
        datapath.coprocessor.fpr[16] = *value2;

        while !datapath.is_halted() {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.coprocessor.condition_code, *expected_result);
        Ok(())
    }
}
