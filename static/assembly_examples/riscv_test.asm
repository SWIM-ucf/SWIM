main:
    jal x1, L1
    bgeu x1, x2, L1
    ret

L1:
    add x1, x2, x3
