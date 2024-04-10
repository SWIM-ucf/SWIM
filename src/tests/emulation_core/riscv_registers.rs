use crate::emulation_core::riscv::registers::{RiscGpRegisterType, RiscGpRegisters};

#[test]
#[allow(clippy::field_reassign_with_default)]
fn direct_access_register() {
    let mut registers = RiscGpRegisters::default();

    registers.pc = 5;

    assert_eq!(registers.pc, 5);
}

#[test]
#[should_panic]
#[allow(unconditional_panic)]
#[allow(clippy::out_of_bounds_indexing)]
fn direct_access_register_bad_gpr() {
    let mut registers = RiscGpRegisters::default();

    registers.gpr[45] = 50;
}

#[test]
fn access_valid_register_by_enum() {
    let mut registers = RiscGpRegisters::default();

    registers[RiscGpRegisterType::X10] = 4;

    assert_eq!(registers.gpr[10], 4);
}

#[test]
fn access_valid_register_by_enum_2() {
    let mut registers = RiscGpRegisters::default();

    registers.gpr[5] = 20;

    assert_eq!(registers[RiscGpRegisterType::X5], 20);
}

#[test]
fn access_valid_register_by_string() {
    let mut registers = RiscGpRegisters::default();

    registers["x31"] = 1;

    assert_eq!(registers.gpr[31], 1);
}

#[test]
fn access_valid_register_by_string_2() {
    let mut registers = RiscGpRegisters::default();

    registers["x24"] = 24;

    assert_eq!(registers.gpr[24], 24);
}

#[test]
#[should_panic]
fn access_bad_register_by_string() {
    let mut registers = RiscGpRegisters::default();

    registers["not_a_real_register"] = 7;
}

#[test]
#[should_panic]
fn no_modify_zero_register_by_enum() {
    let mut registers = RiscGpRegisters::default();

    registers[RiscGpRegisterType::X0] = 5;
}

#[test]
#[should_panic]
fn no_modify_zero_register_by_string() {
    let mut registers = RiscGpRegisters::default();

    registers["zero"] = 90;
}

#[test]
fn registers_into_iter() {
    let mut registers = RiscGpRegisters {
        pc: 500,
        ..Default::default()
    };
    registers.gpr[1] = 19;
    registers.gpr[2] = 45;

    let mut iter = registers.into_iter();

    assert_eq!(Some((RiscGpRegisterType::Pc, 500)), iter.next());
    assert_eq!(Some((RiscGpRegisterType::X0, 0)), iter.next());
    assert_eq!(Some((RiscGpRegisterType::X1, 19)), iter.next());
    assert_eq!(Some((RiscGpRegisterType::X2, 45)), iter.next());
}
