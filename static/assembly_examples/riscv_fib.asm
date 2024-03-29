fib:
 addi sp,sp,-32
 sw ra,28(sp)
 sw s0,24(sp)
 sw s1,20(sp)
 addi s0,sp,32
 sw a0,-20(s0)
 lw a4,-20(s0)
 li a5,1
 blt a5,a4,L2
 lw a5,-20(s0)
 j L3

L2:
 lw a5,-20(s0)
 addi a5,a5,-1
 mv a0,a5
 jalr ra # 26 <.L2+0x8>
 mv s1,a0
 lw a5,-20(s0)
 addi a5,a5,-2
 mv a0,a5
 jalr ra # 38 <.L2+0x1a>
 mv a5,a0
 add a5,a5,s1

L3:
 mv a0,a5
 lw ra,28(sp)
 lw s0,24(sp)
 lw s1,20(sp)
 addi sp,sp,32
 ret

main:
 addi sp,sp,-32
 sw ra,28(sp)
 sw s0,24(sp)
 addi s0,sp,32
 li a5,5
 sw a5,-20(s0)
 lw a0,-20(s0)
 jalr ra # 62 <main+0x12>
 sw a0,-24(s0)
 lw a5,-24(s0)
 mv a0,a5
 lw ra,28(sp)
 lw s0,24(sp)
 addi sp,sp,32
 ret