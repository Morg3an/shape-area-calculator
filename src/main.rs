use std::io;

fn main() {
    println!("Area Calculator");

    const PI: f64 = 3.14159;
    const UNIT: &str = "cm";

    println!("Choose a shape to calculate its area:");
    println!("1. Circle");
    println!("2. Rectangle");
    println!("3. Triangle");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice = choice.trim();

    match choice {
        "1" => {
            println!("Enter the radius of the circle in cm:");
            let mut radius = String::new();
            io::stdin()
                .read_line(&mut radius)
                .expect("Failed to read line");
            let radius: f64 = match radius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };
            let area = PI * radius * radius;
            println!("The area of the circle is {} {}", area, UNIT);
        }
        "2" => {
            println!("Enter the length of the rectangle in cm:");
            let mut length = String::new();
            io::stdin()
                .read_line(&mut length)
                .expect("Failed to read line");
            let length: f64 = match length.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };
            println!("Enter the width of the rectangle in cm:");
            let mut width = String::new();
            io::stdin()
                .read_line(&mut width)
                .expect("Failed to read line");
            let width: f64 = match width.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };
            let area = length * width;
            println!("The area of the rectangle is {} {}", area, UNIT);
        }
        "3" => {
            println!("Enter the base of the triangle in cm:");
            let mut base = String::new();
            io::stdin()
                .read_line(&mut base)
                .expect("Failed to read line");
            let base: f64 = match base.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };
            println!("Enter the height of the triangle in cm:");
            let mut height = String::new();
            io::stdin()
                .read_line(&mut height)
                .expect("Failed to read line");
            let height: f64 = match height.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };
            let area = 0.5 * base * height;
            println!("The area of the triangle is {} {}", area, UNIT);
        }
        _ => {
            println!("Invalid choice! Please enter 1, 2, or 3.");
        }
    }
}
