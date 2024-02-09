use crate::emulation_core::mips::registers::GpRegisterType;

use super::*;

#[test]
fn recursive_fibonacci() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        "        j       main

    fib(int):
            addiu   $sp,$sp,-40
            sw      $ra,36($sp)
            sw      $fp,32($sp)
            sw      $s0,28($sp)
            move    $fp,$sp
            sw      $a0,40($fp)
            lw      $v0,40($fp)
            nop
            addi    $s1,$zero,2
            slt     $v0,$v0,$s1
            beq     $v0,$zero,L2
            nop
    
            lw      $v0,40($fp)
            b       L3
            nop
    
    L2:
            lw      $v0,40($fp)
            nop
            addiu   $v0,$v0,-1
            move    $a0,$v0
            jal     fib(int)
            nop
    
            move    $s0,$v0
            lw      $v0,40($fp)
            nop
            addiu   $v0,$v0,-2
            move    $a0,$v0
            jal     fib(int)
            nop
    
            addu    $v0,$s0,$v0
    L3:
            move    $sp,$fp
            lw      $ra,36($sp)
            lw      $fp,32($sp)
            lw      $s0,28($sp)
            addiu   $sp,$sp,40
            jr      $ra
            nop
    
    main:
            addiu   $sp,$sp,-40
            sw      $ra,36($sp)
            sw      $fp,32($sp)
            move    $fp,$sp
            li      $v0,7            # This is where \"n\" is set to 5
            sw      $v0,24($fp)
            lw      $a0,24($fp)
            jal     fib(int)
            nop
    
            sw      $v0,28($fp)
            lw      $v0,28($fp)      # This is where the final answer gets loaded off the stack
            move    $sp,$fp
            lw      $ra,36($sp)
            lw      $fp,32($sp)
            addiu   $sp,$sp,40
            syscall                  # This was an $ra, changed to syscall to prevent infinite execution
            nop",
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers[GpRegisterType::V0], 13); // fibo(7) is 13

    Ok(())
}
