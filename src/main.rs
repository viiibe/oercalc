use std::env;
use std::io;

#[derive(PartialEq, Eq)]
enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vec: Vec<i32> = vec![];

    if args.len() > 1 {
        let op = match args[1].as_str() {
            "add" => OperationType::Addition,
            "sub" => OperationType::Subtraction,
            "mul" => OperationType::Multiplication,
            "div" => OperationType::Division,
            _ => {
                println!("Please specify arithmetic operation: add/sub/mul/div.");
                return;
            }
        };

        for arg in args {
            let is_division = op == OperationType::Division;

            if let Ok(num) = arg.trim().parse::<i32>() {
                if is_division && !vec.is_empty() && num == 0 {
                    println!("Cannot divide by zero");
                    return;
                }

                vec.push(num);
            }
        }
        if vec.len() == 0 {
            println!("No numbers specified.");
            return;
        }
        println!(
            "Result: {}",
            if vec.len() == 1 {
                vec[0]
            } else {
                calculate(&op, vec)
            }
        );
        return;
    }

    let mut buf = String::new();

    println!(
        "Input operation type\n1 - Addition\n2 - Subtraction\n3 - Multiplication\n4 - Division"
    );

    let op: i32 = loop {
        buf.clear();

        read_user_input(&mut buf);

        if let Ok(n) = buf.trim().parse() {
            if n < 1 || n > 4 {
                println!("Number must be between 1 and 4");
                continue;
            }
            break n;
        } else {
            println!("Not a number, try again");
            continue;
        };
    };

    let op = match op {
        1 => Ok(OperationType::Addition),
        2 => Ok(OperationType::Subtraction),
        3 => Ok(OperationType::Multiplication),
        4 => Ok(OperationType::Division),
        _ => Err("Wrong operation!"),
    };

    let op_str = operation_type_to_string(&op.as_ref().unwrap());
    println!("Input the amount of numbers to input for {op_str}.");

    let num_amount = loop {
        buf.clear();
        read_user_input(&mut buf);

        if let Ok(num) = buf.trim().parse::<usize>() {
            if num < 2 {
                println!("Number must be 2 or bigger");
                continue;
            }

            break num;
        } else {
            println!("Wrong input, try again.");
            continue;
        }
    };

    let mut buf: String = String::new();

    println!("Input the numbers to operate on.");

    if num_amount < 2 {
        panic!("Too few arguments!");
    }

    for i in 1..=num_amount {
        loop {
            buf.clear();
            read_user_input(&mut buf);

            let is_division = if let Ok(o) = &op {
                o == &OperationType::Division
            } else {
                false
            };
            let not_first_element = i > 1;

            if let Ok(num) = buf.trim().parse::<i32>() {
                if is_division && not_first_element && num == 0 {
                    println!("Cannot divide by zero.");
                    continue;
                }
                vec.push(num);
            } else {
                println!("Please input the correct number");
                continue;
            }
            break;
        }
    }

    println!("Result: {}", calculate(&op.as_ref().unwrap(), vec));
}

fn calculate(op: &OperationType, vec: Vec<i32>) -> i32 {
    // TODO: Handle division by zero
    match op {
        OperationType::Addition => vec.iter().copied().sum(),
        OperationType::Subtraction => vec.iter().copied().reduce(|a, b| a - b).unwrap(),
        OperationType::Multiplication => vec.iter().copied().reduce(|a, b| a * b).unwrap(),
        OperationType::Division => vec.iter().copied().reduce(|a, b| a / b).unwrap(),
    }
}

fn read_user_input(buf: &mut String) {
    io::stdin().read_line(buf).expect("Something bad happened");
}

fn operation_type_to_string(op: &OperationType) -> String {
    match op {
        OperationType::Addition => "addition".to_string(),
        OperationType::Subtraction => "subtraction".to_string(),
        OperationType::Multiplication => "multiplication".to_string(),
        OperationType::Division => "division".to_string(),
    }
}
