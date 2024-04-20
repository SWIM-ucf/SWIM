fib(int):
        # Stack variables: | return addr | argument | n - 1 | n - 2 |

        # Save return address and argument
        sw ra, 0(sp)
        sw a0, 4(sp)

        ori t0, zero, 0
        ori t1, zero, 1
        beq a0, t0, RET_0
        beq a0, t1, RET_1

        # Recursive call for fib(n-1)
        addi a0, a0, -1 # Setting arg
        addi sp, sp, -16 # Making space on the stack
        jal ra, fib(int)
        sw a1, 8(sp) # Storing the return value
        lw a0, 4(sp) # Restoring our original argument

        # Recursive call for fib(n-2)
        addi a0, a0, -2
        addi sp, sp, -16
        jal ra, fib(int)
        sw a1, 12(sp)

        # Adding together n-1 and n-2 and returning
        lw t0, 8(sp)
        lw t1, 12(sp)

        add a1, t0, t1
        lw ra, 0(sp)
        addi sp, sp, 16
        #Pseudo-Instruction: ret
        jalr x0, x1, 0 #Pseudo-Instruction Translation


RET_0:
        ori a1, zero, 0
        addi sp, sp, 16
        #Pseudo-Instruction: ret
        jalr x0, x1, 0 #Pseudo-Instruction Translation
RET_1:
        ori a1, zero, 1
        addi sp, sp, 16
        #Pseudo-Instruction: ret
        jalr x0, x1, 0 #Pseudo-Instruction Translation

main:
        ori a0, zero, 5
        addi sp, sp, -16
        jal ra, fib(int)
        ori a0, zero, 1
        ecall
        ori a0, zero, 0
        ecall
