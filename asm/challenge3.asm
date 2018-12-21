; create tree ascii art, argument is tree depth
global _start
tree_char equ '*'
space_char equ ' '

section .data
space_buffer: times 100 db space_char
tree_buffer:  times 100 db tree_char
tree_charset: db '*', '-', '+', '.'
current_index: db 10
newline_buffer: db 0xa

section .text
_start:
mov rdi, rsp
add rdi, 16 ; skip argc and command
mov rdi, [rdi]
call str_to_int
mov r12, rax
call randomize_tree_buffer
mov rdi, r12
mov rsi, 0
call show_triangle
mov rdi, r12
mov rsi, 2
call show_triangle
mov rdi, r12
mov rsi, 3
call show_triangle
mov rdi, r12
call show_trunk
; exit
xor rdi, rdi
mov rax, 60
syscall

randomize_tree_buffer:
mov rdi, 0
mov rsi, 4
loop_random:
imul rax, [current_index], 5
add rax, 7
mov [current_index], al
xor rdx, rdx
div rsi
mov cl, [tree_charset+rdx]
mov [tree_buffer+rdi], cl
inc rdi
cmp rdi, 100
jl loop_random
ret

show_triangle:
; rdi height, rsi lines skipped
mov r9, rdi
mov r10, 1
sub r9, rsi
add rsi, rsi
add r10, rsi
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
ret

show_trunk:
mov r9, rdi
mov r10, rdi
shr r10, 1
inc r10
mov r12, rdi
trunk_loop:
mov rdi, r10
call print_n_space
mov rdi, r9
call print_n_tree
call print_newline
dec r12
cmp r12, 0
jnz trunk_loop
ret

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
