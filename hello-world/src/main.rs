fn main() {
    println!("Hello, world!");

    let x = 4; // by defaut it's immutabel to make it mutable add the keyword mut

    println!("The value of x is {}", x);

    let mut y = 5; // this is how to declare a mutable variable 

    println!("The value of y is {}", y);

    y= 10;

    println!("The reassigned value of y is {}", y);

}
