main:
        # Testing FPU R4 Type Operations!
        # All results are made positive since displays/prints always display unsigned forms and it gets weird.
        # All expected output noted in comments.
        # Set Print Flag
        ori a0, zero, 1

        # Initialize Initial Value
        addi t0, zero, 5
        addi t1, zero, 7
        addi t2, zero, 10


        # Convert to FP
        fcvt.s.w ft0, t0
        fcvt.s.w ft1, t1
        fcvt.s.w ft2, t2

        # Test fmadd (Should return 45)
        fmadd.s ft3, ft0, ft1, ft2
        fcvt.w.s a1, ft3
        ecall

        # Test fmsub (Should return 25)
        fmsub.s ft4, ft0, ft1, ft2
        fcvt.w.s a1, ft4
        ecall

        # Test fnmsub (Should return 10)
        fnmsub.s ft5, ft0, ft1, ft3
        fcvt.w.s a1, ft5
        ecall

        # Test fnmadd (Should return 5)
        fnmadd.s ft6, ft0, ft1, ft0
        fadd.s ft7, ft6, ft3
        fcvt.w.s a1, ft7
        ecall

        # Reset Flags
        ori a0, zero, 0
        ecall
