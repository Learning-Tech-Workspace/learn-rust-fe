use std::{fs::File, io::Write};

fn main() {
    let wat = wasmprinter::print_file(
        "/home/asus/Workspace/rust/rust-wasm/target/wasm32-unknown-unknown/debug/rust-test.wasm",
    )
    .unwrap();
    File::create("rust-test.wat")
        .unwrap()
        .write_all(wat.as_bytes())
        .unwrap();
}

#[no_mangle]
pub extern "C" fn add(num1: u64, num2: u64) -> u64 {
    num1 + num2
}

#[no_mangle]
pub extern "C" fn mul(num1: u64, num2: u64) -> u64 {
    num1 * num2
}
