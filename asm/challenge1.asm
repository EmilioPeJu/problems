; sum numbers from argv (variable length)
global _start
section .text
_start:
mov r8, [rsp] ; argc
mov r9, rsp  
add r9, 8 ; argv
xor r10, r10 ; aux for storing max
sub r8, 1
add r9, 8 ; skip command
loop_args:
cmp r8, byte 0
jz end
mov rdi, r9
mov rdi, [rdi]
call str_to_int
cmp rax, r10
jl skip_max
mov r10, rax
skip_max
sub r8, 1
add r9, 8
jmp loop_args
end:
mov rdi, r10
mov rax, 60
syscall

str_to_int:
; rdi is a pointer to string
xor rax, rax
jmp cond1
loop_digs:
imul rax, rax, 10
mov rdx, [rdi]
and rdx, 0xff
add rax, rdx
sub rax, 48
add rdi, 1
cond1:
cmp [rdi], byte 0
jnz loop_digs
ret
