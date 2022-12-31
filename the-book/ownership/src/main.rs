fn main() {
    let s = "hello";

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);


    {
    let x = 5;
    let y = x;
    }

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}", s1);


    let a = {
        let mut b = String::from("hello");
        b.push_str(" world");
        b
    };
    println!("{a}");


    let s = String::from("hello");

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;

    makes_copy(x);


    let s = String::from("hello");
    let s2;
    let b = false;
    if b {
        s2 = s;
    }
    //    println!("{s}");



    // References and Borrowing

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);



    // Giving error

    //let s = String::from("hello");
    //change(&s);


    let mut s = String::from("hello");
    print(&s);
    s.push_str(" world");
    print(&s);


    // Mutable references

    let mut s = String::from("hello");

    change_1(&mut s);
    change_2(&mut s);

    println!("{s}")
}



fn change_1(some_string: &mut String){
    some_string.push_str(", world 1");
}

fn change_2(some_string: &mut String){
    some_string.push_str(", world 2");
}

fn print(s: &String){
    println!("{s}");
}

//fn change(some_string: &String){
//    some_string.push_str(", world");
//}

fn calculate_length(s: &String)->usize{
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
