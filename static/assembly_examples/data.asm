.text
addi $t0, $zero, 300
addi $t1, $zero, 340
addi $t2, $zero, 380
syscall
.data
a_secret: .ascii "hi"
importantQuestion: .asciiz "What is system software?"
numbers: .byte  0x64, 100, 'a', 'b'
half: .half 1000
buffer: .space 8
word: .word 12345678
float: .float 1.20
double: .double 1.22222
