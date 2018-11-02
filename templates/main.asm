.ORG     $0000    // IPLエリア
         call    M
         halt

.ORG     $1000    // ユーザエリア

{{ main }}

P:
   mov r1, $200
   mov *(r1), r0
   ret
