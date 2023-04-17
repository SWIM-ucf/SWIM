# The goal is to load the value 0xABCD 8765 CCCC EEEE
# into register R1.

# Sample tracing:
#    Instruction    |     Register R1     |     Notes
# ------------------+---------------------+-----------------------------
# lui r1, 52428     | FFFF FFFF CCCC 0000 | 52428 == 0xCCCC. C == 1100, so the value is sign-extended.
# ori r1, r1, 61166 | FFFF FFFF CCCC EEEE | 61166 == 0xEEEE.
# dahi r1, 34662    | FFFF 8765 CCCC EEEE | 34662 == 0x8766. FFFF + 8766 = 8765.
# dati r1, 43982    | ABCD 8765 CCCC EEEE | 43982 == 0xABCE. FFFF + ABCE = ABCD.

lui r1, 52428
ori r1, r1, 61166
dahi r1, 34662
dati r1, 43982
