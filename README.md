# Shape Area Calculator

A simple Rust command-line application that calculates the area of various shapes, including circles, rectangles, and triangles. This project demonstrates the use of variables, constants, and shadowing in Rust.

## Features
- Calculate the area of a:
  - Circle (using the constant value of PI).
  - Rectangle (using user-provided length and width).
  - Triangle (using user-provided base and height).
- Input validation to ensure valid numeric values.
- User-friendly interface with clear prompts and output.

## Learning Objectives
This project was created as part of a 30-day Rust learning challenge and covers:
- Declaring and using variables (mutable and immutable).
- Defining constants with the `const` keyword.
- Implementing shadowing to reuse variable names for updated values.

## How to Use
1. Clone the repository:
   ```bash
   git clone https://github.com/Morg3an/shape-area-calculator.git
   ```
2. Navigate to the project directory:
   ```bash
   cd shape-area-calculator
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the project:
   ```bash
   cargo run
   ```
5. Follow the on-screen instructions to select a shape and provide dimensions.

## Example Output
```
Area Calculator
Choose a shape to calculate its area:
1. Circle
2. Rectangle
3. Triangle

Enter your choice: 1
Enter the radius of the circle in cm: 5
The area of the circle is 78.53975 cm
```

## Project Structure
```
src
├── main.rs       # Main application logic
Cargo.toml        # Project metadata and dependencies
```

## Dependencies
This project uses only the Rust standard library, so no additional dependencies are required.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
