main:
        # Testing FPU Arithmetic and I/O Works Using Pythagorean Theorem
        # Set Print Flag
        ori a0, zero, 5

        # Enter first value and then store to t0
        ecall
        or t0, a1, zero

        # Enter second value and then store to t1
        ecall
        or t1, a1, zero

        # Set SYSCALL Flag to Print
        ori a0, zero, 1

        # Convert to Floating Point
        fcvt.s.w ft0, t0
        fcvt.s.w ft1, t1

        # Print Minimum
        fmin.s ft2, ft0, ft1
        fcvt.w.s a1, ft2
        ecall

        # Print Maximum
        fmax.s ft3, ft0, ft1
        fcvt.w.s a1, ft3
        ecall

        # Print Squares (Multiplication)
        fmul.s ft4, ft0, ft0
        fcvt.w.s a1, ft4
        ecall

        fmul.s ft5, ft1, ft1
        fcvt.w.s a1, ft5
        ecall

        # Print Addition
        fadd.s ft6, ft4, ft5
        fcvt.w.s a1, ft6
        ecall

        # Print Subtraction
        fsub.s ft7, ft6, ft1
        fcvt.w.s a1, ft7
        ecall

        # Print Division
        fdiv.s ft0, ft7, ft0
        fcvt.w.s a1, ft0
        ecall

        # Test Sqrt and Print Hypotenuse
        fsqrt.s ft7, ft6
        fcvt.w.s a1, ft7
        ecall

        # Reset Print Flag
        ori a0, zero, 0
        ecall
