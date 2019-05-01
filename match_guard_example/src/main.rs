fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        // while it may look like the 'if y' only applies to the 6 case, it really applies to 4, 5 AND 6
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
