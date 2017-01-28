global _start
extern input
extern print
extern exit


section .text
_start:
    call input
    neg rax
    push rax
    call input
    pop rdi
    add rdi, rax
    call print
    call exit
