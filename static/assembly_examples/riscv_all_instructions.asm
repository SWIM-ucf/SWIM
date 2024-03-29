lui x1, 0x12345      # Load Upper Immediate: Load 0x12345000 into register x1
auipc x2, 0x1000     # Add Upper Immediate to PC: Add 0x1000 to the current PC and store the result in x2
addi x3, x4, 100     # Add Immediate: Add 100 to the value in register x4 and store the result in x3
slti x5, x6, 10      # Set Less Than Immediate: Set x5 to 1 if x6 < 10; otherwise, set x5 to 0
xori x7, x8, 0xFF    # Bitwise XOR Immediate: Perform bitwise XOR between x8 and 0xFF and store the result in x7
ori x9, x10, 0xABC   # Bitwise OR Immediate: Perform bitwise OR between x10 and 0xABC and store the result in x9
andi x11, x12, 0xF0  # Bitwise AND Immediate: Perform bitwise AND between x12 and 0xF0 and store the result in x11
slli x13, x14, 2     # Shift Left Logical Immediate: Shift the value in x14 left by 2 bits and store the result in x13
srli x15, x16, 3     # Shift Right Logical Immediate: Shift the value in x16 right by 3 bits and store the result in x15
srai x17, x18, 4     # Shift Right Arithmetic Immediate: Shift the value in x18 right arithmetically by 4 bits and store the result in x17
add x19, x20, x21    # Add: Add the values in x20 and x21 and store the result in x19
sub x22, x23, x24    # Subtract: Subtract the value in x24 from x23 and store the result in x22
sll x25, x26, x27    # Shift Left Logical: Shift the value in x26 left by the number of bits specified in x27 and store the result in x25
slt x28, x29, x30    # Set Less Than: Set x28 to 1 if x29 < x30; otherwise, set x28 to 0
sltu x31, x1, x2     # Set Less Than Unsigned: Set x31 to 1 if x1 < x2 (unsigned comparison); otherwise, set x31 to 0
srl x6, x7, x8       # Shift Right Logical: Shift the value in x7 right by the number of bits specified in x8 and store the result in x6
sra x9, x10, x11     # Shift Right Arithmetic: Shift the value in x10 right arithmetically by the number of bits specified in x11 and store the result in x9
or x12, x13, x14     # Bitwise OR: Perform bitwise OR between x13 and x14 and store the result in x12
and x15, x16, x17    # Bitwise AND: Perform bitwise AND between x16 and x17 and store the result in x15
ecall                # Environment Call: Raise an environment call exception
ebreak               # Breakpoint: Trigger a breakpoint exception
uret                 # User Return: Return from user mode to the previous privilege mode
sret                 # Supervisor Return: Return from supervisor mode to the previous privilege mode
mret                 # Machine Return: Return from machine mode to the previous privilege mode
wfi
lb x1, 100(x2)       # Load Byte: Load a signed byte from memory at the address specified by x2 + 100 and sign-extend it to fill a 32-bit register x1
lh x3, 200(x4)       # Load Halfword: Load a signed halfword from memory at the address specified by x4 + 200 and sign-extend it to fill a 32-bit register x3
lw x5, 300(x6)       # Load Word: Load a word from memory at the address specified by x6 + 300 and store it in x5
lbu x7, 400(x8)      # Load Byte Unsigned: Load an unsigned byte from memory at the address specified by x8 + 400 and zero-extend it to fill a 32-bit register x7
lhu x9, 500(x10)     # Load Halfword Unsigned: Load an unsigned halfword from memory at the address specified by x10 + 500 and zero-extend it to fill a 32-bit register x9
sb x11, 600(x12)     # Store Byte: Store the lowest byte of x11 to memory at the address specified by x12 + 600
sh x13, 700(x14)     # Store Halfword: Store the lowest halfword of x13 to memory at the address specified by x14 + 700
sw x15, 800(x16)     # Store Word: Store the value in x15 to memory at the address specified by x16 + 800
jal x17, 0x100       # Jump and Link: Jump to the address 0x100 and store the address of the instruction following the jump in register x17
jalr x18, x19, 0x200 # Jump and Link Register: Jump to the address in x19 + 0x200 and store the address of the instruction following the jump in register x18
beq x20, x21, 0x300  # Branch if Equal: If x20 is equal to x21, jump to the address of the instruction at 0x300
bne x22, x23, 0x400  # Branch if Not Equal: If x22 is not equal to x23, jump to the address of the instruction at 0x400
blt x24, x25, 0x500  # Branch if Less Than: If x24 is less than x25, jump to the address of the instruction at 0x500
bge x26, x27, 0x600  # Branch if Greater Than or Equal: If x26 is greater than or equal to x27, jump to the address of the instruction at 0x600
bltu x28, x29, 0x700 # Branch if Less Than Unsigned: If x28 is less than x29 (unsigned comparison), jump to the address of the instruction at 0x700
bgeu x30, x31, 0x800 # Branch if Greater Than or Equal Unsigned: If x30 is greater than or equal to x31 (unsigned comparison), jump to the address of the instruction at 0x800
addiw x1, x2, 100    # Add Immediate Word: Add 100 to the value in x2 and store the 32-bit result in x1
slliw x3, x4, 3       # Shift Left Logical Immediate Word: Shift the value in x4 left by 3 bits and store the 32-bit result in x3
srliw x5, x6, 2       # Shift Right Logical Immediate Word: Shift the value in x6 right by 2 bits and store the 32-bit result in x5
addw x7, x8, x9       # Add Word: Add the values in x8 and x9 and store the 32-bit result in x7
subw x10, x11, x12    # Subtract Word: Subtract the value in x12 from x11 and store the 32-bit result in x10
sllw x13, x14, x15    # Shift Left Logical Word: Shift the value in x14 left by the number of bits specified in x15 and store the 32-bit result in x13
srlw x16, x17, x18    # Shift Right Logical Word: Shift the value in x17 right by the number of bits specified in x18 and store the 32-bit result in x16
sraw x19, x20, x21    # Shift Right Arithmetic Word: Shift the value in x20 right arithmetically by the number of bits specified in x21 and store the 32-bit result in x19
lwu x22, 100(x23)     # Load Word Unsigned: Load an unsigned word from memory at the address specified by x23 + 100 and zero-extend it to fill a 32-bit register x22
ld x24, 200(x25)      # Load Doubleword: Load a doubleword from memory at the address specified by x25 + 200 and store it in x24
sd x26, 300(x27)      # Store Doubleword: Store the value in x26 to memory at the address specified by x27 + 300
mul x1, x2, x3        # Multiply: Multiply the values in x2 and x3 and store the 64-bit result in registers x1 and x2
mulh x4, x5, x6       # Multiply High: Multiply the values in x5 and x6, then return the upper 64 bits of the 128-bit result in x4
mulhsu x7, x8, x9     # Multiply High Signed Unsigned: Multiply the signed value in x8 by the unsigned value in x9, then return the upper 64 bits of the 128-bit result in x7
mulhu x10, x11, x12   # Multiply High Unsigned: Multiply the unsigned values in x11 and x12, then return the upper 64 bits of the 128-bit result in x10
div x13, x14, x15     # Divide: Divide the value in x14 by the value in x15, then store the quotient in x13
divu x16, x17, x18    # Divide Unsigned: Divide the unsigned value in x17 by the unsigned value in x18, then store the quotient in x16
rem x19, x20, x21     # Remainder: Calculate the remainder when dividing the value in x20 by the value in x21, then store the result in x19
remu x22, x23, x24    # Remainder Unsigned: Calculate the unsigned remainder when dividing the value in x23 by the value in x24, then store the result in x22
mulw x25, x26, x27    # Multiply Word: Multiply the values in x26 and x27 and store the 32-bit result in x25
divw x28, x29, x30    # Divide Word: Divide the value in x29 by the value in x30, then store the 32-bit quotient in x28
divuw x31, x1, x2     # Divide Unsigned Word: Divide the unsigned value in x1 by the unsigned value in x2, then store the 32-bit quotient in x31
remw x3, x4, x5       # Remainder Word: Calculate the remainder when dividing the value in x4 by the value in x5, then store the 32-bit result in x3
remuw x6, x7, x8      # Remainder Unsigned Word: Calculate the unsigned remainder when dividing the value in x7 by the value in x8, then store the 32-bit result in x6
fmadd.s f1, f2, f3, f4  # Fused Multiply-Add Single: Compute (f2 * f3) + f4 and store the result in f1
fmsub.s f5, f6, f7, f8  # Fused Multiply-Subtract Single: Compute (f6 * f7) - f8 and store the result in f5
fnmsub.s f9, f10, f11, f12  # Fused Negate Multiply-Subtract Single: Compute -(f10 * f11) - f12 and store the result in f9
fnmadd.s f13, f14, f15, f16  # Fused Negate Multiply-Add Single: Compute -(f14 * f15) + f16 and store the result in f13
fadd.s f17, f18, f19    # Floating-Point Add Single: Compute f18 + f19 and store the result in f17
fsub.s f20, f21, f22