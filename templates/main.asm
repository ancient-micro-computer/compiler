.ORG     $0000    // IPLエリア
         call    _MAIN
         halt

.ORG     $1000    // ユーザエリア
_MAIN:
{{ main }}
    pop r0
    mov r1, $200
    mov *(r1), r0
    ret
