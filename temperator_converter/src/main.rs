use std::io;
fn main() {
    println!("Temperature converter enter 0 to convert c to f and 1 to convert f to c");
    
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");
    
    let unit_to_convert = if input.trim() == "0" {"Celsius"} else {"Fahrenheit"};
    let converted_unit = if input.trim() == "0" {"Fahrenheit"} else {"Celsius"};
    
    println!("Please enter {unit_to_convert} unit the value you want to convert");
    
    
    
    let mut temp_to_convert = String::new();
    
    io::stdin()
        .read_line(&mut temp_to_convert)
        .expect("Faild to read the temperature line");
    
    let temp_to_convert: i64 = temp_to_convert
        .trim()
        .parse()
        .expect("Failed to parse into Int");
    
    let result = match input.trim() {
        "0" => c_to_f(temp_to_convert),
        "1" => f_to_c(temp_to_convert),
         _ => panic!("Invalid entered convertion unit"),
    };
    
    
    println!("{unit_to_convert} {temp_to_convert} is {result} in {converted_unit}");
    
}

fn c_to_f(c: i64) -> i64 {
    c * 9/5 + 32
    
}

fn f_to_c(f: i64) -> i64 {
    (f - 32) * 5/9
}
