# To contribute to this project https://github.com/SWIM-ucf
# 
# Delay slots are not implemented for this emulator, keep this in
# mind when writing Jump or Branch instructions. For information
# on delay slots read https://en.wikipedia.org/wiki/Delay_slot.
#
# Make sure to checkout the interactive datapath diagram.



# This demo code was generated using gcc 12.2 at https://godbolt.org/:
        j       main            # Make sure to manually add this line for gcc generated code

fib(int):
        addiu   $sp,$sp,-40
        sw      $31,36($sp)
        sw      $fp,32($sp)
        sw      $16,28($sp)
        move    $fp,$sp
        sw      $4,40($fp)
        lw      $2,40($fp)
        nop
        slt     $2,$2,2
        beq     $2,$0,$L2
        nop

        lw      $2,40($fp)
        b       $L3
        nop

$L2:
        lw      $2,40($fp)
        nop
        addiu   $2,$2,-1
        move    $4,$2
        jal     fib(int)
        nop

        move    $16,$2
        lw      $2,40($fp)
        nop
        addiu   $2,$2,-2
        move    $4,$2
        jal     fib(int)
        nop

        addu    $2,$16,$2
$L3:
        move    $sp,$fp
        lw      $31,36($sp)
        lw      $fp,32($sp)
        lw      $16,28($sp)
        addiu   $sp,$sp,40
        jr      $31
        nop

main:
        addiu   $sp,$sp,-40
        sw      $31,36($sp)
        sw      $fp,32($sp)
        move    $fp,$sp
        li      $2,5            # This is where "n" is set
        sw      $2,24($fp)
        lw      $4,24($fp)
        jal     fib(int)
        nop

        sw      $2,28($fp)
        lw      $2,28($fp)      # This is where the final answer gets loaded off the stack
        move    $sp,$fp
        lw      $31,36($sp)
        lw      $fp,32($sp)
        addiu   $sp,$sp,40
        jr      $31
        nop

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