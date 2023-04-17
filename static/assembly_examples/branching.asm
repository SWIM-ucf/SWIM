# This program loops 5 times, increasing register $s1 by multiples of
# 12345 on each iteration.

# $s0 stores the iteration count.
# $s1 stores the value 12345.
# $s2 stores the value 5 (for ending the loop).
# $s3 stores the value being incremented.

or $s0, $zero, $zero
ori $s1, $zero, 12345
ori $s2, $zero, 5
or $s3, $zero, $zero
loop: daddu $s3, $s3, $s1
daddiu $s0, $s0, 1
bne $s0, $s2, loop
