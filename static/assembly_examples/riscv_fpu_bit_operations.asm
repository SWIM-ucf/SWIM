main:
        # Testing FPU FMV by Moving Bit Patterns and Manipulating Them Across Both Paths
        # All expected output noted in comments.
        # Set Print Flag
        ori a0, zero, 1

        # Initialize Initial Value
        addi t0, zero, 3

        # Convert to FP
        fcvt.s.w ft0, t0

        # Sqrt (Check FP Register for 1.7... value)
        fsqrt.s ft1, ft0

        # Move Bit Pattern to Integer
        fmv.x.w t1, ft1

        # Negate
        addi t2, zero, 0x1
        slli t3, t2, 31
        or t4, t1, t3

        # Move Back To FP (Check FP Register Again)
        fmv.w.x ft2, t4

        # Square
        fmul.s ft3, ft2, ft2

        # EQ to Original Value (Should Return 1)
        feq.s a1, ft0, ft3
        ecall

        # LT to Original Value (Should Return 0)
        flt.s a1, ft3, ft0
        ecall

        # LE to Original Value (Should Return 1)
        fle.s a1, ft3, ft0
        ecall

        # LE to Lesser Value (Should Return 0)
        fle.s a1, ft3, ft1
        ecall

        # Classify ft2 (Negative, should return 2 (0010))
        fclass.s a1, ft2
        ecall

        # Classify ft3 (Positive, should return 64 (0100 0000))
        fclass.s a1, ft3
        ecall

        # Reset Print Flag
        ori a0, zero, 0
        ecall
