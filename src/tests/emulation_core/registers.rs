use crate::emulation_core::mips::registers::{RegisterType, Registers};

#[test]
#[allow(clippy::field_reassign_with_default)]
fn direct_access_register() {
    let mut registers = Registers::default();

    registers.pc = 5;

    assert_eq!(registers.pc, 5);
}

#[test]
#[should_panic]
#[allow(unconditional_panic)]
fn direct_access_register_bad_gpr() {
    let mut registers = Registers::default();

    registers.gpr[45] = 50;
}

#[test]
#[should_panic]
#[allow(unconditional_panic)]
fn direct_access_register_bad_fpr() {
    let mut registers = Registers::default();

    registers.fpr[32] = 4;
}

#[test]
fn access_valid_register_by_enum() {
    let mut registers = Registers::default();

    registers[RegisterType::T2] = 4;

    assert_eq!(registers.gpr[10], 4);
}

#[test]
fn access_valid_register_by_enum_2() {
    let mut registers = Registers::default();

    registers.gpr[5] = 20;

    assert_eq!(registers[RegisterType::A1], 20);
}

#[test]
fn access_valid_register_by_string() {
    let mut registers = Registers::default();

    registers["cc"] = 1;

    assert_eq!(registers.cc, 1);
}

#[test]
fn access_valid_register_by_string_2() {
    let mut registers = Registers::default();

    registers["f20"] = 24;

    assert_eq!(registers.fpr[20], 24);
}

#[test]
#[should_panic]
fn access_bad_register_by_string() {
    let mut registers = Registers::default();

    registers["not_a_real_register"] = 7;
}
