fn main() {
    // conditional statements 
    let num = 2;

    if num == 1 {
        println!("you won!");
    } else if num == 2{
        println!("Try again!");
    } else {
        println!("you lost!");
    }

    // looping statement 

    let mut x = 2;

    loop {
        x = x * 2;

        if x >= 5000 {
            break;
        }

        println!("Value of x is {}", x);
    }
    println!("Out of loop!");


    //  while loop

    let mut y = 2;

    while y < 5000{
        y = y * 2;
        println!("the value of y is {}", y);
    }

    //  for loops
    for x in 0..10{ // exclusive range 0-9
        println!("The value of x is {}", x);
    }

    for y in 0..=10{
        println!("The value of y is {}", y);
    }


    let i = [1,2,3];

    for values in i {
        println!("The value is {}", values);
    }

}
