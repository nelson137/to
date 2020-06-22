global _start

section .data
text db "hello from asm", 10
len equ $ - text

section .text
_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, text
    mov rdx, len
    syscall
    mov rax, 60
    xor rdi, rdi
    syscall
