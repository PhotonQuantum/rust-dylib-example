#[no_mangle]
pub extern fn is_odd(x: usize) -> bool {
    x % 2 == 1
}