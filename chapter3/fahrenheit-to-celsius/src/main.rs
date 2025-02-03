use std::io::{stdin, stdout, Write};

fn main() {
    let mut degrees_f = String::new();

    let degrees_f: f64 = loop {
        print!("Degrees in Fahrenheit: ");
        stdout().flush().expect("Error: Could not flush stdout");
        degrees_f.clear();

        stdin()
            .read_line(&mut degrees_f)
            .expect("Error: Could not read from stdin");

        match degrees_f.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Error: Could not parse input");
                continue;
            }
        }
    };

    let degrees_c = convert_f_to_c(degrees_f);
    let formatted_degrees_c = format!("{:.2}", degrees_c);

    println!("Degrees in Celsius: {formatted_degrees_c}");
}

fn convert_f_to_c(degrees_f: f64) -> f64 {
    return (degrees_f - 32.0) * (5.0 / 9.0);
}
