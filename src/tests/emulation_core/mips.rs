use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::emulation_core::mips::registers::RegisterType;

#[allow(clippy::unusual_byte_groupings)]
#[test]
fn add_register_to_itself() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // T1 = T1 + T1
    //                       R-type  T1    T1    T1  (shamt)  ADD
    let instruction: u32 = 0b000000_01001_01001_01001_00000_100000;
    datapath.memory.store_word(0, instruction)?;

    // Assume the T1 register has the value 5.
    datapath.registers[RegisterType::T1] = 5;

    datapath.execute_instruction();

    // After the operation is finished, the register should be 10.
    (datapath.registers[RegisterType::T1] == 10)
        .then_some(())
        .ok_or_else(|| String::from("Unexpected value in register."))
}
