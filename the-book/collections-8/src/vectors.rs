pub fn working_with_vectors() {
    
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1,2,3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(4);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    } // This way is prefered when the bounds on the vector may be bigger than 
    // the size of the vector.



    // Iterating over the values in a vector

    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!{"{i}"}
    }

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}