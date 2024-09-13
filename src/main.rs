use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> Result<f64, &'static str> {
    match op {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0.0 {
                Err("Sıfıra bölme hatası!")
            } else {
                Ok(a / b)
            }
        }
    }
}

fn get_number(prompt: &str) -> Result<f64, std::io::Error> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Geçersiz sayı formatı"))
}

fn get_operation() -> Result<char, std::io::Error> {
    println!("İşlemi girin (+, -, *, /):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().chars().next().ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Geçersiz işlem",
    ))
}

fn main() {
    let num1 = match get_number("İlk sayıyı girin:") {
        Ok(num) => num,
        Err(e) => {
            println!("Hata: {}", e);
            return;
        }
    };

    let op = match get_operation() {
        Ok(op) => op,
        Err(e) => {
            println!("Hata: {}", e);
            return;
        }
    };

    let num2 = match get_number("İkinci sayıyı girin:") {
        Ok(num) => num,
        Err(e) => {
            println!("Hata: {}", e);
            return;
        }
    };

    let operation = match op {
        '+' => Operation::Add(num1, num2),
        '-' => Operation::Subtract(num1, num2),
        '*' => Operation::Multiply(num1, num2),
        '/' => Operation::Divide(num1, num2),
        _ => {
            println!("Geçersiz işlem!");
            return;
        }
    };

    match calculate(operation) {
        Ok(result) => println!("Sonuç: {}", result),
        Err(e) => println!("Hata: {}", e),
    }
}
