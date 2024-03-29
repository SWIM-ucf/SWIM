main:
    addi x5, x5, 36
    fcvt.s.w f3, x5
    fsqrt.s f4, f3
    fcvt.w.s x6, f4
    ecall