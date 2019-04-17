use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let alt_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores {
        println!("scores - team {} has {} points", key, value);
    }

    for (key, value) in &alt_scores {
        println!("alt_scores - team {} has {} points", key, value);
    }
    
    // An example of entry at play
    let mut entry_scores = HashMap::new();
    entry_scores.insert(String::from("Blue"), 10);

    entry_scores.entry(String::from("Yellow")).or_insert(50);
    entry_scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // An example of using entry to update value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // Count here is a mutable reference if the value is already set
        let count = map.entry(word).or_insert(0);
        // Dereference the reference and increment value
        *count += 1;
    }

    println!("{:?}", map);
}
