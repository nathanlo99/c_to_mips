
; in:  $1 = base pointer
; in:  $2 = array length
; out: $3 = height

height:
  lis $6
  .word helper
  lis $7
  .word 4
  lis $8
  .word -1
  add $2, $0, $0
  jr $6           ; Tail call: helper will return to caller for us

; in  : $1 = base pointer
; in  : $2 = index
; out : $3 = height
; registers - $1 unchanged, $2 destroyed, $3 returned, $4 $5 preserved
helper:
  ; Push $4, $5
  sw $4, -4($30)
  sw $5, -8($30)
  sw $31, -12($30)
  lis $4
  .word 12
  sub $30, $30, $4

  ; Handle -1 case
  bne $2, $8, nonzero
  add $3, $0, $0
  beq $0, $0, helperEnd

nonzero:
  mult $2, $7
  mflo $2
  add $4, $2, $1      ; $4 = ARR + 4 * IDX

  lw $2, 4($4)        ; $2 = ARR[IDX + 1]
  jalr $6             ; $3 = height(this.left)
  add $5, $3, $0      ; $5 = $3

  lw $2, 8($4)        ; $2 = ARR[IDX + 2]
  jalr $6             ; $3 = height(this.right)

  ; Return the larger of $3 and $5
  slt $2, $5, $3
  beq $2, $0, helperEnd
  add $3, $5, $0 

helperEnd:
  ; Pop $4, $5
  lis $4
  .word 12
  add $30, $30, $4
  lw $4, -4($30)
  lw $5, -8($30)
  lw $31, -12($30)
  jr $31
