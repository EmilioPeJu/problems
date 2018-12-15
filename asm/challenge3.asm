; create tree ascii art, argument is tree depth
global _start
tree_char equ '*'
space_char equ ' '

section .data
space_buffer: times 100 db space_char
tree_buffer:  times 100 db tree_char
newline_buffer: db 0xa

section .text
_start:
mov rdi, rsp
add rdi, 16 ; skip argc and command
mov rdi, [rdi]
call str_to_int
mov r9, rax
mov r10, 1
jmp cond_show
loop_show:
mov rdi, r9
call print_n_space
mov rdi, r10
call print_n_tree
call print_newline
dec r9
add r10, 2
cond_show:
cmp r9, 0
jge loop_show
; exit
mov rdi, rax
mov rax, 60
syscall

print_n_space:
mov rax, 1
mov rdx, rdi 
mov rdi, 1
mov rsi, space_buffer
syscall
ret

print_n_tree:
mov rax, 1
mov rdx, rdi 
mov rdi, 1
mov rsi, tree_buffer
syscall
ret

print_newline:
mov rax, 1
mov rdi, 1
mov rsi, newline_buffer
mov rdx, 1
syscall
ret

str_to_int:
xor rax, rax
mov rdx, 10
jmp cond_div10
loop_div10:
mov sil, byte [rdi]
sub sil, 0x30
imul rax, rdx
add rax, rsi
inc rdi
cond_div10:
cmp [rdi], byte 0
jnz loop_div10
ret
