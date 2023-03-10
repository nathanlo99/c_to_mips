; Takes a twos complement integer in register $1 and prints it out to stdout
; storing to 0xffff000c prints to stdout
; loading from 0xffff0004 grabs from stdin

; Setup constants
; $8 = stdout
; $9 = - = 45
; $10 = 0 = 48
; $11 = 10
; $12 = 4

print:
preamble:
; Save registers onto stack
sw $1, -4($30)
sw $2, -8($30)
sw $3, -12($30)
sw $8, -16($30)
sw $9, -20($30)
sw $10, -24($30)
sw $11, -28($30)
sw $12, -32($30)
sw $29, -36($30)
lis $2
.word 36
sub $30, $30, $2

constants:
lis $8
.word 0xffff000c
lis $9
.word 45
lis $10
.word 48
lis $11
.word 10
lis $12
.word 4

; $29 = old SP
add $29, $30, $0

bne $1, $0, nonzero
; Zero case
sw $10, 0($8)           ; print 0
beq $0, $0, postamble

nonzero:
slt $2, $1, $0          ; $2 is 1 iff num is negative
beq $2, $0, positive    ; If $2 == 1, then print a minus sign and negate $1
sw $9, 0($8)            ; print -
sub $1, $0, $1          ; negate $1

positive:
beq $1, $0, printStack  ; if num == 0, done
divu $1, $11            ; lo = num / 10, hi = num % 10
mfhi $1                 ; $1 = remainder
add $1, $1, $10         ; $1 now has the character
sub $30, $30, $12       ; SP -= 4
sw $1, 0($30)           ; push character onto stack
mflo $1                 ; $1 = quotient
beq $0, $0, positive    ; loop

printStack:
beq $29, $30, postamble
lw $1, 0($30)           ; grab character from stack
add $30, $30, $12       ; SP += 4
sw $1, 0($8)            ; print character
beq $0, $0, printStack  ; loop

postamble:
; Grab saved registers from stack and jump back
lis $2
.word 36
add $30, $30, $2
lw $1, -4($30)
lw $2, -8($30)
lw $3, -12($30)
lw $8, -16($30)
lw $9, -20($30)
lw $10, -24($30)
lw $11, -28($30)
lw $12, -32($30)
lw $29, -36($30)
jr $31
