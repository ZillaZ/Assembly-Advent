std::arch::global_asm!(
    r#"
.global fill_numbers
fill_numbers:
push rbp
mov rbp, rsp

read_file:
mov QWORD PTR[rbp-48], rdx
mov QWORD PTR[rbp-8], rsi
mov QWORD PTR[rbp-56], rcx
xor rcx, rcx
mov QWORD PTR[rbp-64], rcx
mov rax, 2
mov rsi, 64
mov rdx, 420
syscall
mov rdi, rax
xor rax, rax
mov rsi, QWORD PTR[rbp-8]
mov rdx, 100000
syscall
sub rax, 1
mov QWORD PTR[rbp-16], rax
xor rax, rax
xor rdx, rdx
xor rdi, rdi
xor rbx, rbx

check:
mov QWORD PTR[rbp-24], rax
mov rax, QWORD PTR[rbp-16]
cmp rbx, rax
mov rax, QWORD PTR[rbp-24]
je rtn
jmp read_line

read_line:
xor rdx, rdx
movzx rdx, BYTE PTR[rsi]
sub rdx, '0'
cmp rdx, 9
ja next
add rbx, 1
inc rsi
imul rax, 10
add rax, rdx
jmp check

list_two:
mov rbx, QWORD PTR[rbp-56]
lea rbx, [rbx+rdi*8]
mov QWORD PTR[rbx], rax
xor rax, rax
mov rbx, QWORD PTR[rbp-64]
sub rbx, 1
mov QWORD PTR[rbp-64], rbx
mov rbx, QWORD PTR[rbp-40]
add rdi, 1
jmp loop

next:
mov QWORD PTR[rbp-40], rbx
mov rbx, QWORD PTR[rbp-64]
cmp rbx, 1
je list_two
add rbx, 1
mov QWORD PTR[rbp-64], rbx
mov rbx, QWORD PTR[rbp-48]
lea rbx, [rbx+rdi*8]
mov QWORD PTR[rbx], rax
xor rax, rax
mov rbx, QWORD PTR[rbp-40]

loop:
xor rdx, rdx
movzx rdx, BYTE PTR[rsi]
sub rdx, '0'
cmp rdx, 9
jbe check
add rbx, 1
inc rsi
jmp loop

rtn:
mov QWORD PTR[rbp-40], rbx
mov rbx, QWORD PTR[rbp-56]
lea rbx, [rbx+rdi*8]
mov QWORD PTR[rbx], rax
pop rbp
ret
"#
);

std::arch::global_asm!(
    r#"
.global total_diff
total_diff:
xor rax, rax
sub rdx, 1

add_loop:
cmp rdx, 0
jl end
mov rbx, QWORD PTR[rdi+rdx*8]
mov r10, QWORD PTR[rsi+rdx*8]
cmp rbx, r10
jl diff
sub rdx, 1
sub rbx, r10
add rax, rbx
jmp add_loop

diff:
mov rbx, QWORD PTR[rdi+rdx*8]
mov r10, QWORD PTR[rsi+rdx*8]
sub r10, rbx
add rax, r10
sub rdx, 1
jmp add_loop

end:
ret
"#
);

extern "C" {
    fn fill_numbers(
        filepath: *const i8,
        buffer: *mut u8,
        numbers: *mut u64,
        numbers_two: *mut u64,
    ) -> u64;
    fn total_diff(one: *mut u64, two: *mut u64, size: usize) -> i64;
}

fn main() {
    unsafe {
        let filename = std::ffi::CString::new("input.txt").unwrap();
        let mut buf = [0; 100000];
        let mut numbers = [0; 100000];
        let mut numbers_two = [0; 100000];
        let _bytes_read = fill_numbers(
            filename.as_ptr(),
            buf.as_mut_ptr(),
            numbers.as_mut_ptr(),
            numbers_two.as_mut_ptr(),
        );
        let mut end = numbers.len() - 1;
        while numbers[end] == 0 {
            end -= 1;
        }
        let numbers = &mut numbers[0..=end];
        let mut end = numbers_two.len() - 1;
        while end > 0 && numbers_two[end] == 0 {
            end -= 1;
        }
        let numbers_two = &mut numbers_two[0..=end];
        numbers.sort();
        numbers_two.sort();
        let total = total_diff(
            numbers.as_mut_ptr(),
            numbers_two.as_mut_ptr(),
            numbers.len(),
        );
        println!("{total}");
    }
}
