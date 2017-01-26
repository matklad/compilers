global _start
extern input
extern print

section .text
_start:
    call input
    xor rdi, rdi
    add rdi, rax
    add rdi, rax
    call print

    mov eax, 60
    xor rdi, rdi
    syscall
message:
    db "Hello, World!", 10
