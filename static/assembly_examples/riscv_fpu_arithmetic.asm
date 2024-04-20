main:
        # Testing FPU Arithmetic Works Using Pythagorean Theorem
        # All expected output noted in comments.
        # Set Print Flag
        ori a0, zero, 1

        # Initialize Initial Values
        addi t0, zero, 3
        addi t1, zero, 4

        # Convert to Floating Point
        fcvt.s.w ft0, t0
        fcvt.s.w ft1, t1

        # Test Minimum (3)
        fmin.s ft2, ft0, ft1
        fcvt.w.s a1, ft2
        ecall

        # Test Maximum (4)
        fmax.s ft3, ft0, ft1
        fcvt.w.s a1, ft3
        ecall

        # Test Multiplication (9, 16)
        fmul.s ft4, ft0, ft0
        fcvt.w.s a1, ft4
        ecall

        fmul.s ft5, ft1, ft1
        fcvt.w.s a1, ft5
        ecall

        # Test Addition (25)
        fadd.s ft6, ft4, ft5
        fcvt.w.s a1, ft6
        ecall

        # Test Subtraction (21)
        fsub.s ft7, ft6, ft1
        fcvt.w.s a1, ft7
        ecall

        # Test Division (7)
        fdiv.s ft0, ft7, ft0
        fcvt.w.s a1, ft0
        ecall

        # Test Sqrt and Return Hypotenuse (5)
        fsqrt.s ft7, ft6
        fcvt.w.s a1, ft7
        ecall

        # Reset Print Flag
        ori a0, zero, 0
        ecall
