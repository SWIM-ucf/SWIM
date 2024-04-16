add $t1, $zero, $v0
addi $t1, $t0, 10
addiu $t1, $t0, 100
addu $t1, $s0, $s1
and $t1, $t2, $t3
andi $t1, $t2, 255
b label
beq $t0, $t1, label
bne $t0, $t1, label
dadd $t1, $s0, $s1
daddi $t1, $t0, 4
daddiu $t1, $t0, 10
daddu $t1, $s0, $s1
dahi $t1, 15
dati $t1, 15
ddiv $t0, $t1, $t2
ddivu $t0, $t1, $t2
div $t0, $t1, $t2
dmul $t0, $t1, $t2
dmulu $t0, $t1, $t2
dsub $t1, $s0, $s1
dsubu $t1, $s0, $s1
j label
jal label
jalr $t0, $t1
jr $t0
lui $t1, 65535
lw $t0, 0($t1)
mul $t0, $t1, $t2
nop
or $t1, $t2, $t3
ori $t1, $t2, 255
sll $t1, $t0, 10
slt $t1, $s0, $s1
sltu $t1, $s0, $s1
sub $t1, $s0, $s1
sw $t0, 0($t1)
syscall
