fn main() {
    let sum =my_function(11, 22);
    println!("The sum is: {}", sum);
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y // returns sum (inside a function, the last expression is returned)
}
