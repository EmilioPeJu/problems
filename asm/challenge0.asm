; reverse a string passed as argument
global _start
section .data
newline: db 10
buffer: times 100 db 0
section .text
_start:
mov rax, rsp
add rax, 16
mov rax, [rax]
mov rbx, rax
jmp cond1
loop1:
add rbx, 1
cond1:
cmp [rbx], byte 0
jnz loop1
mov rcx, rbx
sub rcx, rax
mov r9, rcx
shr rcx, 1
sub rbx, 1
jmp cond2
loop2:
mov rdx, [rbx]
mov r10, [rax]
mov [rbx], r10b
mov [rax], dl
add rax, 1
sub rbx, 1
sub rcx, 1
cond2:
cmp rcx, 0
ja loop2

mov rsi, rsp
add rsi, 16
mov rsi, [rsi]
mov rdi, buffer
jmp cond3
loop3:
mov r11, [rsi]
mov [rdi], r11b
inc rsi
inc rdi
cond3:
cmp [rsi], byte 0
ja loop3

mov rdi, 1
mov rsi, buffer
mov rdx, r9
mov rax, 1
syscall

mov rdi, 1
mov rsi, newline
mov rdx, 1
mov rax, 1
syscall

xor rdi, rdi
mov rax, 60
syscall
