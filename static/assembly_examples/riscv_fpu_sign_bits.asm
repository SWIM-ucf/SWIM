main:
        # Since Printing Negatives To The Console is a bit Temperamental at the moment, use the Registers with Float mode to check values.
        # Sign Bit Operations Check

        addi t0, zero, 13
        addi t1, zero, 7
        addi t2, zero, -5
        addi t3, zero, -3

        fcvt.s.w ft0, t0
        fcvt.s.w ft1, t1
        fcvt.s.w ft2, t2
        fcvt.s.w ft3, t3

        # Check Sign Bit Normal (Should be Negative 13 in Register)
        fsgnj.s ft4, ft0, ft2
        # Check Sign Bit Negation (Should be Negative 7 in Register)
        fsgnjn.s ft5, ft1, ft0
        # Check Sign Bit Xor Both (Should be Positive 5 in Register)
        fsgnjx.s ft6, ft2, ft3
        # Check Sign Bit Xor Neither (Should be Positive 13 in Register)
        fsgnjx.s ft7, ft0, ft1
        # Check Sign Bit Xor One (Should be Negative 3 in Register)
        fsgnjx.s ft8, ft3, ft0

        ecall
