use std::collections::VecDeque;
use std::f64::consts::PI;
use std::fs::OpenOptions;
use std::io::{self, Write};
use csv::{Reader, Writer};
use regex::Regex;

const HISTORY_SIZE: usize = 10;
const CSV_FILE: &str = "history.csv";

fn main() {
    let mut history: VecDeque<String> = VecDeque::with_capacity(HISTORY_SIZE);
    load_history_from_csv(&mut history);

    loop {
        println!("\n--- Command-Line Calculator ---");
        println!("1. Arithmetic Operations");
        println!("2. Unit Conversion");
        println!("3. Number System Conversion");
        println!("4. View History");
        println!("5. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => handle_arithmetic_operations(&mut history),
            "2" => handle_unit_conversion(&mut history),
            "3" => handle_number_system_conversion(&mut history),
            "4" => display_history(&history),
            "5" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn handle_arithmetic_operations(history: &mut VecDeque<String>) {
    print!("Enter expression: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();

    // Handle expressions without spaces using regex
    let re = Regex::new(r"([+\-*/])").unwrap();
    let processed = re.replace_all(trimmed, " $1 ").to_string();

    match evaluate_expression(&processed) {
        Ok(result) => {
            println!("Result: {}", result);
            update_history(history, trimmed, &result.to_string());
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn handle_unit_conversion(history: &mut VecDeque<String>) {
    println!("\n--- Unit Conversion ---");
    println!("1. Bytes to KB");
    println!("2. KB to Bytes");
    println!("3. Miles to Kilometers");
    println!("4. Kilometers to Miles");
    println!("5. Celsius to Fahrenheit");
    println!("6. Fahrenheit to Celsius");
    println!("7. Kilograms to Pounds");
    println!("8. Pounds to Kilograms");

    let (conversion_type, value) = match get_conversion_input() {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let result = match conversion_type.as_str() {
        "1" => value / 1024.0,
        "2" => value * 1024.0,
        "3" => value * 1.60934,
        "4" => value / 1.60934,
        "5" => value * 9.0 / 5.0 + 32.0,
        "6" => (value - 32.0) * 5.0 / 9.0,
        "7" => value * 2.20462,
        "8" => value / 2.20462,
        _ => {
            println!("Invalid choice.");
            return;
        }
    };

    let operation = format!("Unit Conversion: {} ({})", get_unit_conversion_name(&conversion_type), value);
    println!("Result: {:.2}", result);
    update_history(history, &operation, &format!("{:.2}", result));
}

fn handle_number_system_conversion(history: &mut VecDeque<String>) {
    println!("\n--- Number System Conversion ---");
    println!("1. Binary to Decimal");
    println!("2. Binary to Octal");
    println!("3. Binary to Hexadecimal");
    println!("4. Decimal to Binary");
    println!("5. Decimal to Octal");
    println!("6. Decimal to Hexadecimal");
    println!("7. Octal to Decimal");
    println!("8. Octal to Binary");
    println!("9. Octal to Hexadecimal");
    println!("10. Hexadecimal to Decimal");
    println!("11. Hexadecimal to Binary");
    println!("12. Hexadecimal to Octal");

    let (conversion_type, input) = match get_number_conversion_input() {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let result = match conversion_type.as_str() {
        "1" => convert_number(&input, 2, 10),
        "2" => convert_number(&input, 2, 8),
        "3" => convert_number(&input, 2, 16),
        "4" => convert_number(&input, 10, 2),
        "5" => convert_number(&input, 10, 8),
        "6" => convert_number(&input, 10, 16),
        "7" => convert_number(&input, 8, 10),
        "8" => convert_number(&input, 8, 2),
        "9" => convert_number(&input, 8, 16),
        "10" => convert_number(&input, 16, 10),
        "11" => convert_number(&input, 16, 2),
        "12" => convert_number(&input, 16, 8),
        _ => {
            println!("Invalid choice.");
            return;
        }
    };

    match result {
        Ok(output) => {
            let operation = format!("Number Conversion: {} ({})", 
                get_number_conversion_name(&conversion_type), input);
            println!("Result: {}", output);
            update_history(history, &operation, &output);
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    
    match tokens.len() {
        1 => match tokens[0] {
            "pi" => Ok(PI),
            _ => tokens[0].parse().map_err(|_| "Invalid number".into()),
        },
        2 => {
            let operand: f64 = tokens[1].parse().map_err(|_| "Invalid number")?;
            match tokens[0] {
                "sin" => Ok(operand.sin()),
                "cos" => Ok(operand.cos()),
                "tan" => Ok(operand.tan()),
                "log" => Ok(operand.ln()),
                _ => Err("Unknown operation".into()),
            }
        }
        3 => {
            let left: f64 = tokens[0].parse().map_err(|_| "Invalid number")?;
            let right: f64 = tokens[2].parse().map_err(|_| "Invalid number")?;
            match tokens[1] {
                "+" => Ok(left + right),
                "-" => Ok(left - right),
                "*" => Ok(left * right),
                "/" => {
                    if right == 0.0 {
                        Err("Division by zero".into())
                    } else {
                        Ok(left / right)
                    }
                }
                _ => Err("Unknown operator".into()),
            }
        }
        _ => Err("Invalid expression format".into()),
    }
}

fn convert_number(input: &str, from_base: u32, to_base: u32) -> Result<String, String> {
    u32::from_str_radix(input, from_base)
        .map_err(|_| format!("Invalid input for base {}", from_base))
        .map(|decimal| match to_base {
            2 => format!("{:b}", decimal),
            8 => format!("{:o}", decimal),
            10 => decimal.to_string(),
            16 => format!("{:x}", decimal).to_uppercase(),
            _ => unreachable!(),
        })
}

fn update_history(history: &mut VecDeque<String>, expression: &str, result: &str) {
    let entry = format!("{} = {}", expression, result);
    if history.len() >= HISTORY_SIZE {
        history.pop_front();
    }
    history.push_back(entry.clone());
    save_to_csv(expression, result).unwrap_or_else(|e| eprintln!("CSV Error: {}", e));
}

fn display_history(history: &VecDeque<String>) {
    println!("\n--- History (Last {} entries) ---", HISTORY_SIZE);
    for entry in history {
        println!("{}", entry);
    }
}

fn save_to_csv(expression: &str, result: &str) -> Result<(), csv::Error> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(CSV_FILE)?;

    let mut writer = Writer::from_writer(file);
    writer.write_record(&[expression, result])?;
    writer.flush()?;
    Ok(())
}

fn load_history_from_csv(history: &mut VecDeque<String>) {
    if let Ok(mut reader) = Reader::from_path(CSV_FILE) {
        for result in reader.records() {
            if let Ok(record) = result {
                if record.len() == 2 {
                    let entry = format!("{} = {}", &record[0], &record[1]);
                    if history.len() >= HISTORY_SIZE {
                        history.pop_front();
                    }
                    history.push_back(entry);
                }
            }
        }
    }
}

// Helper functions
fn get_unit_conversion_name(choice: &str) -> &'static str {
    match choice {
        "1" => "Bytes to KB",
        "2" => "KB to Bytes",
        "3" => "Miles to Kilometers",
        "4" => "Kilometers to Miles",
        "5" => "Celsius to Fahrenheit",
        "6" => "Fahrenheit to Celsius",
        "7" => "Kilograms to Pounds",
        "8" => "Pounds to Kilograms",
        _ => "Unknown Conversion",
    }
}

fn get_number_conversion_name(choice: &str) -> &'static str {
    match choice {
        "1" => "Binary to Decimal",
        "2" => "Binary to Octal",
        "3" => "Binary to Hexadecimal",
        "4" => "Decimal to Binary",
        "5" => "Decimal to Octal",
        "6" => "Decimal to Hexadecimal",
        "7" => "Octal to Decimal",
        "8" => "Octal to Binary",
        "9" => "Octal to Hexadecimal",
        "10" => "Hexadecimal to Decimal",
        "11" => "Hexadecimal to Binary",
        "12" => "Hexadecimal to Octal",
        _ => "Unknown Conversion",
    }
}

fn get_conversion_input() -> Result<(String, f64), String> {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).map_err(|e| e.to_string())?;
    let choice = choice.trim().to_string();

    print!("Enter value to convert: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    
    let mut value = String::new();
    io::stdin().read_line(&mut value).map_err(|e| e.to_string())?;
    let value: f64 = value.trim().parse::<f64>().map_err(|_| "Invalid number".to_string())?;


    Ok((choice, value))
}

fn get_number_conversion_input() -> Result<(String, String), String> {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).map_err(|e| e.to_string())?;
    let choice = choice.trim().to_string();

    print!("Enter value to convert: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    let input = input.trim().to_string();

    Ok((choice, input))
}