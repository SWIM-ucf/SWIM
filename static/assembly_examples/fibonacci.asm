# Delay slots are not implemented for this emulator, keep this in
# mind when writing Jump or Branch instructions. For information
# on delay slots read https://en.wikipedia.org/wiki/Delay_slot.
#
# Make sure to checkout the interactive datapath diagram equip with mouse hover information.

# This demo code was generated using gcc 12.2 at https://godbolt.org/:
fib(int):
        addiu   $sp,$sp,-40
        sw      $ra,36($sp)
        sw      $fp,32($sp)
        sw      $s0,28($sp)
        move    $fp,$sp
        sw      $a0,40($fp)
        lw      $v0,40($fp)
        nop
        addi    $s1,$zero,2
        slt     $v0,$v0,$s1
        beq     $v0,$zero,L2
        nop

        lw      $v0,40($fp)
        b       L3
        nop

L2:
        lw      $v0,40($fp)
        nop
        addiu   $v0,$v0,-1
        move    $a0,$v0
        jal     fib(int)
        nop

        move    $s0,$v0
        lw      $v0,40($fp)
        nop
        addiu   $v0,$v0,-2
        move    $a0,$v0
        jal     fib(int)
        nop

        addu    $v0,$s0,$v0
L3:
        move    $sp,$fp
        lw      $ra,36($sp)
        lw      $fp,32($sp)
        lw      $s0,28($sp)
        addiu   $sp,$sp,40
        jr      $ra
        nop

main:                           # PC starts here
        addiu   $sp,$sp,-40
        sw      $ra,36($sp)
        sw      $fp,32($sp)
        move    $fp,$sp
        li      $v0,5            # This is where "n" is set
        sw      $v0,24($fp)
        lw      $a0,24($fp)
        jal     fib(int)         # fibo(5)
        nop

        sw      $v0,28($fp)
        lw      $v0,28($fp)      # This is where the final answer gets loaded off the stack
        move    $sp,$fp
        lw      $ra,36($sp)
        lw      $fp,32($sp)
        addiu   $sp,$sp,40
        li      $a0, 0           # Load 0 into $a0 to run the exit syscall
        syscall                  # replaced jr $ra with syscall to prevent infinite execution

# Here's the demo code in C:
#
#static int fib(int n){
#    if (n <= 1)
#        return n;
#    return fib(n - 1) + fib(n - 2);
#}
# 
#int main(){
#    int n = 5;
#    int ans = fib(n);
#    return ans;
#}