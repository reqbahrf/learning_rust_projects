use std::io;


fn main() {
    println!("Input a number to get the nth of a Fibonacci sequence");
    
    let mut nth_of = String::new();
    
    io::stdin()
        .read_line(&mut nth_of)
        .expect("Failed to read line");
    

    let nth = get_nth_num(nth_of);
    println!("The int num is: {nth}");
}


fn get_nth_num(input: String) -> u32 {
    let mut counter: u32 = 0;
    let mut a = 0;
    let mut b = 1;
    let mut temp = 0;
    let target_nth: u32 = input
        .trim()
        .parse()
        .expect("Failed to parse into int");
    
    while counter < target_nth - 2 {
      temp = a + b;
      a = b;
      b = temp; 
        
      println!("The temp is: {temp}");
        
      counter += 1
        
    };
    
    temp
    
}
