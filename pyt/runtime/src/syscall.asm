    global syscall1
    global syscall3
    section .text

syscall1:
    mov rax, rdi
    mov rdi, rsi
    syscall
    ret

syscall3:
    mov rax, rdi
    mov rdi, rsi
    mov rsi, rdx
    mov rdx, rcx
    syscall
    ret