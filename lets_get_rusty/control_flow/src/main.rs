fn main() {
    // Control Flow
    let condition = true;
    let number = if condition {5} else {6};
    let number = 5;

    if number < 10 { //if statements must be boolean
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }
    let mut counter = 0;
    let result = loop { //loop
        counter += 1; 

        if counter ==10 {
            break counter;
        }
    };//needs semicolon at the end of the loop

    println!("The result is {}", result);

    let mut number = 3;

    while number !=0 { // while the number is not zero, execute the code in this loop
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() { //for every element in our array, print that element to the screen
        println!("the value is: {}", element);
    }

    for number in 1..4 { //for every number in this range, take the number and print it out
        println!("{}!", number);
    }
}    


