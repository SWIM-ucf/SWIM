.data
a_secret: .ascii " 🥸 "
importantQuestion: .asciiz "What is system software?"
numbers: .byte  0x64, 1000, 'a', 'b'
half: .half 1000
buffer: space 8
word: .word 12345678
float: .float 1.20
double: .double 1.22222

.text
li $t0, 100
lw $t1, 🥸
move $t22, $t1
lw $t2, important_question
syscall
