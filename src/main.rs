fn main() {
    let x = 15;
    {
        let x = x + 2;
        let y = 51;
        println!("x = {}, y = {}", x, y); // prints 17, 51
    }
    // println!("x = {}, y = {}", x, y); // Error
}
