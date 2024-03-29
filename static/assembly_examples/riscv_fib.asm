fib(int):
        # t0 and t1 are the last two values of the fib sequence
        ori t0, zero, 0 # a_n-2
        ori t1, zero, 1 # a_n-1
        ori t2, zero, 1 # a_n
        ori t3, zero, 1 # index

COND:
        bne a0, t3, LOOP # If we've reached our index, return
        or a1, zero, t2
        #Pseudo-Instruction: ret
        jalr x0, x1, 0 #Pseudo-Instruction Translation

LOOP:
        # Legendary fibonacci calculation
        add t2, t0, t1
        or t0, zero, t1
        or t1, zero, t2
        addi t3, t3, 1
        jal x0, COND
        
main:
        ori a0, zero, 15
        jal ra, fib(int)
        ori a0, zero, 1
        ecall
        ori a0, zero, 0
        ecall