fib:
        # Stack variables: | return addr | argument | n - 1 | n - 2 |

        # Save return address and argument
        sw x1, 0(x2)
        sw x10, 4(x2)

        ori x5, x0, 0
        ori x6, x0, 1
        beq x10, x5, RET_0
        beq x10, x6, RET_1

        # Recursive call for fib(n-1)
        addi x10, x10, -1 # Setting arg
        addi x2, x2, -16 # Making space on the stack
        jalr x1, x5, 0
        sw x11, 8(x2) # Storing the return value
        lw x10, 4(x2) # Restoring our original argument

        # Recursive call for fib(n-2)
        addi x10, x10, -2
        addi x2, x2, -16
        jalr x1, x5, 0
        sw x11, 12(x2)

        # Adding together n-1 and n-2 and returning
        lw x5, 8(x2)
        lw x6, 12(x2)

        add x11, x5, x6
        lw x1, 0(x2)
        addi x2, x2, 16
        # Pseudo-Instruction: ret
        jalr x0, x1, 0 # Pseudo-Instruction Translation


RET_0:
        ori x11, x0, 0
        addi x2, x2, 16
        # Pseudo-Instruction: ret
        jalr x0, x1, 0 # Pseudo-Instruction Translation
RET_1:
        ori x11, x0, 1
        addi x2, x2, 16
        # Pseudo-Instruction: ret
        jalr x0, x1, 0 # Pseudo-Instruction Translation

main:
        ori x10, x0, 5
        addi x2, x2, -16
        jalr x1, x5, 0
        ori x10, x0, 1
        ecall
        ori x10, x0, 0
        ecall
