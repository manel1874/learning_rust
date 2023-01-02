use std::collections::HashMap;

pub fn working_with_hashmaps() {
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name)
                        .copied()
                        .unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Now, at this point we cannot use field_name and field_value because they
    // are owned by the map hashmap
    
    // Overwriting a value

    let mut scores1 = HashMap::new();

    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Blue"), 25);

    scores1.entry(String::from("Yellow")).or_insert(50);
    scores1.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores1);

    // Update based on the old value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }

    println!("{:?}", map);

}
