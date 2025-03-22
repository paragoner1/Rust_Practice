#[derive(Debug)]
struct Robot {
    name: String,
    power: u32,
    active: bool,
}
fn main() {
    let name = String::from("Rusty");
    let power = 100;
    let robot1 = Robot { name, power, active: false };
    let robot2 = Robot {
        name: robot1.name.clone(),
        active: true,
        ..robot1
    };
    println!("Robot 1: {:?}", robot1);
    println!("Robot 2: {} is active: {}", robot2.name, robot2.active);
}