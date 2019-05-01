fn main() {
    let mut num = 5;

    // we know that these raw pointers are valid as they are created directly from references guarenteed to be valid
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // an example of unsafely referencing an arbitrary location in memory
    let address = 0x012345usize;
    let r = address as *const i32;

    // when dereferencing raw poitners, need to use `unsafe` block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
