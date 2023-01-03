pub fn panic_function() {
    panic!("Crash and burn you world!\n");
}

pub fn panic_from_other() {
    let v = vec![1,2,3];
    v[99];
}

