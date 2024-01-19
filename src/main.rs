fn main() {
    let yellow: i32;
    if true {
        yellow = 27
    } else {
        yellow = 42
    }
    println!("yellow = {}", yellow);
    temp_scope();
}

fn temp_scope() {
    let x = 15;
    {
        let x = x + 2;
        let y = 51;
        println!("x = {}, y = {}", x, y); // prints 17, 51
    }
    println!("x = {}", x); // Error
}
