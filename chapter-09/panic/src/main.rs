fn main() {
    // // Will just crash with a message
    // panic!("Crash and burn!");

    // Will crash and point to a file we didn't write
    // Use RUST_BACKTRACE to trace the crash
    // export RUST_BACKTRACE=1
    let v = vec![1, 2, 3];
    v[99];
}
