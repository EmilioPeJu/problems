; simplest reverse polish notation calculator
global _start
opplus equ '+'
opminus equ '-'

section .data
char_buffer: db 0
int_buffer: times 100 db 0
end_int_buffer:

section .text
_start:
mov rbp, rsp
mov r12, [rbp]
dec r12
mov r13, rsp
add r13, 16

jmp cond2
loop2:
mov rdi, [r13]
cmp byte [rdi], opplus
je do_sum
cmp byte [rdi], opminus
je do_sub
call str_to_int
push rax

init2:
add r13, 8
dec r12
cond2:
cmp r12, 0
ja loop2

; get result as return code until
; I complete some functions
pop rdi
mov rax, 60
syscall

; routines going back to loop
do_sum:
pop rdi
pop rsi
add rdi, rsi
push rdi
jmp init2

do_sub:
pop rdi
pop rsi
sub rdi, rsi
push rdi
jmp init2

; functions
; not finished
int_to_str:
mov rsi, end_int_buffer
dec rsi
mov byte [rsi], 0
dec rsi
xor rdx, rdx
mov rcx, 10
mov rax, rdi
jmp cond_div10
loop_div10

div rcx
mov rdi, rax
cond_div10:
cmp rdi,0
ja loop_div10

ret

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


print_char:
mov [char_buffer], dil
mov rdi, 1
mov rsi, char_buffer
mov rdx, 1
mov rax, 1
syscall
ret

