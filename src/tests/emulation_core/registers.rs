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

#[test]
#[should_panic]
fn no_modify_zero_register_by_enum() {
    let mut registers = Registers::default();

    registers[RegisterType::Zero] = 5;
}

#[test]
#[should_panic]
fn no_modify_zero_register_by_string() {
    let mut registers = Registers::default();

    registers["zero"] = 90;
}

#[test]
fn registers_into_iter() {
    let mut registers = Registers {
        pc: 500,
        ..Default::default()
    };
    registers.gpr[1] = 19;
    registers.gpr[2] = 45;

    let mut iter = registers.into_iter();

    assert_eq!(Some((RegisterType::Pc, 500)), iter.next());
    assert_eq!(Some((RegisterType::Zero, 0)), iter.next());
    assert_eq!(Some((RegisterType::At, 19)), iter.next());
    assert_eq!(Some((RegisterType::V0, 45)), iter.next());
}
