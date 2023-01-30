
start:
  lis $6
  .word start   ; Should be 0

next:
  lis $6
  .word next    ; Should be 8

longJumpStart:
  beq $0, $0, longJumpEnd ; Should be 5
  add $1, $2, $3
  add $1, $2, $3
  add $1, $2, $3
  add $1, $2, $3
  add $1, $2, $3

longJumpEnd:
  add $1, $2, $3
  beq $0, $0, longJumpStart ; Should be -8
