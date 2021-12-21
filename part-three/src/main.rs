fn main() {
//    matching statements

    let x = 1;

    match x {
        1 => println!("the value is 1"),
        2 => println!("the value is 2"),
        _ => println!("The value is invalid")
    }

    let y = true;
    let j = true;

    match (y,j){
        (true, true) => println!("Both are true"),
        (false, false) => println!("Both are false"),
        (true, false) => println!("y is true and j is false"),
        (false, true) => println!("y is false and j is true")
        
    }

}
