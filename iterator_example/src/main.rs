fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1_mapped: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("v1_mapped: {:?}", v1_mapped);
}
