; simplest reverse polish notation calculator
global _start
opplus equ '+'
opminus equ '-'
opby equ '*'

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
cmp byte [rdi], opby
je do_by
call str_to_int
push rax

init2:
add r13, 8
dec r12
cond2:
cmp r12, 0
ja loop2

call int_to_str
mov rax, 1
mov rdi, 1
mov rsi, rsp
mov rdx, r11
syscall
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
pop rsi
pop rdi
sub rdi, rsi
push rdi
jmp init2

do_by:
pop rdi
pop rsi
imul rdi, rsi
push rdi
jmp init2

; functions
; not finished
int_to_str:
pop r12
xor rdx, rdx
xor r11, r11
mov rcx, 10
mov rax, rdi
push byte 0

jmp cond_div10
loop_div10

div rcx
add rdx, 0x30
dec rsp
mov  [rsp], dl
inc r11
xor rdx, rdx
cond_div10:
cmp rax, 0
ja loop_div10
push r12
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

