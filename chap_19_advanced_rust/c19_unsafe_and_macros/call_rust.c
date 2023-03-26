// Compile with
// $ gcc -g call_rust.c -o call_rust -lc19_unsafe_and_macros -L./c19_unsafe_and_macros/target/debug

int main() {
    // pass char to Rust
    rust_char('A');
    rust_wchar(L'ζ');

    // pass short to Rust
    rust_short(-100);
    rust_ushort(100);

    // pass int to Rust
    rust_int(-10);
    rust_uint(10);

    // pass long to Rust
    rust_long(-1000);
    rust_ulong(1000);

    // pass a void* pointer
    void *p = malloc(1000);
    rust_void(p);

    return 0;
}

//
// `ldd call_rust`
//
// Produces

/*
        linux-vdso.so.1 (0x00007fff6e5b8000)
        ===> ***libc19_unsafe_and_macros.so => not found***  <===
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f429c5a1000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f429c7af000)
*/

//    |
//    | librustcalls.so is reported not found because not in the standard paths for
//    | shared libs. It's necessary to tell the linker where to find this file, using
//    | the LD_LIBRARY_PATH environment variable:
//    |
//
// `LD_LIBRARY_PATH=./target/debug ldd call_rust`
//
// To execute:
//
// `LD_LIBRARY_PATH=./target/debug ./call_rust`