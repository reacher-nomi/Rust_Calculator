Command-Line Calculator

This is a simple command-line calculator written in Rust that supports:

Arithmetic operations

Unit conversions

Number system conversions

History tracking (stored in a CSV file)

Features

Basic arithmetic operations (+, -, *, /) with support for trigonometric and logarithmic functions

Conversion between different number systems (binary, decimal, octal, hexadecimal)

Unit conversions (bytes, distance, temperature, weight)

History tracking of calculations (last 10 entries stored in history.csv)

 https://github.com/your-username/command-line-calculator.git

Installation

To run this project, ensure you have Rust installed.

Clone the repository:

git clone
cd command-line-calculator

Website Link:

https://reacher-nomi.github.io/Rust_Calculator/


Build the project:

cargo build --release

Run the calculator:

cargo run

Usage

Upon running, the program presents a menu:

Arithmetic Operations: Enter expressions like 3 + 4 * 2 or use functions like sin 30

Unit Conversion: Convert between various units like miles to kilometers

Number System Conversion: Convert numbers between binary, decimal, octal, and hexadecimal

View History: Displays the last 10 calculations stored in history.csv

Exit: Close the application

Dependencies

This project uses:

csv for reading/writing history

regex for expression parsing

License

This project is licensed under the MIT License.

Contribution

Feel free to fork the repository and submit pull requests!
