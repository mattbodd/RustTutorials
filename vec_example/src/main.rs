// Remember that you cannot have both an immutable and mutable reference in the same scope for the
// same item
// Also remember that you cannot have more than one mutable reference in the same scope
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &mut v[0];

    // Here we are passing first into the println macro and effectively transfering ownership so
    // first leaves the current scope and by proximity so does the mutable reference to 'mut v[0]'
    println!("The first element is: {}", first);

    v.push(6);
}
