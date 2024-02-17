use crate::emulation_core::mips::instruction::get_string_version;

#[test]
fn get_string_version_from_binary() {
    let instruction: u32 = 0b00110100000011100000000111110100;

    assert!(match get_string_version(instruction) {
        Ok(string) => {
            string.contains("ori $t6 $zero 500")
        }
        _ => false,
    });
}
#[test]
fn get_string_version_from_hex() {
    let instruction: u32 = 0x340e01f4;

    assert!(match get_string_version(instruction) {
        Ok(string) => {
            string.contains("ori $t6 $zero 500")
        }
        _ => false,
    });
}

#[test]
fn err_on_empty_instruction() {
    let instruction: u32 = 0b00000000000000000000000000000000;

    assert!(match get_string_version(instruction) {
        Err(e) => e.contains("empty instruction"),
        _ => false,
    });
}
