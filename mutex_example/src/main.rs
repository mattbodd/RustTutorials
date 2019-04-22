use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    // this contrived scope will show how a mutex lock can be acquired, implicitly released and in between the inner value can be manipulated
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
