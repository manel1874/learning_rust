pub fn working_with_strings() {
    // Strings are a collection of bytes

    let mut s1 = String::new();

    let data = "initial contents";

    let s1 = data.to_string();

    println!("{s1}");

    let mut s2 = String::from("foo");
    s2.push_str("bar");



    // Use format

    let s3 = String::from("tic");
    let s4 = String::from("tac");
    let s5 = String::from("toe");

    let s6 = format!("{s3}-{s4}-{s5}");

    println!("{s6}")

}