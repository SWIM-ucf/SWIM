main:
        # Initialize Original Value
        addi a1, zero, 49
        # Print Original Value
        ori a0, zero, 1
        ecall
        # Convert to Floating Point
        fcvt.s.w ft0, a1
        # Take Square Root (7)
        fsqrt.s ft1, ft0
        # Store/Load Test (Both Processors). Ensure the value is kept the same across all transitions.
        fsw ft1, 0x100(zero)
        lw t0, 0x100(zero)
        sw t0, 0x110(zero)
        flw ft2, 0x110(zero)
        # Reconvert to Integer
        fcvt.w.s a1, ft2
        # Print out Value (Should be ==7)
        ecall
        # Reset Print Register and Return
        ori a0, zero, 0
        ecall
