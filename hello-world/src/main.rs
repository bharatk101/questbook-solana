fn main() {
    println!("Hello, world!");

    let x = 4; // by defaut it's immutabel to make it mutable add the keyword mut

    println!("The value of x is {}", x);

    let mut y = 5; // this is how to declare a mutable variable 

    println!("The value of y is {}", y);

    y= 10;

    println!("The reassigned value of y is {}", y);

    // declare static variable type
    let a: i32 = 4;
    println!("The value of a is {}", a);

    // tuples in rust
    //  tuple arwe set of values of any type

    let c = (5, "Hello", true);
    println!("The first value of tuple c is {}", c.0);

    let (value_one, value_two, _) = c;
    println!("The second value of tuple c is {}", value_two);


    // arrays
    // arrays are set of values of same type

    let u = [1,2,3,4,5];
    println!("The second value of array u is {}", u[1]);

    // declaring array of single element repeated multiple times
    let o = [0; 10]; // the first element is the value and the second element is the number of times you want to repeat

    println!("Array o = {:?}", o); //{:?} printing the entire array



}
