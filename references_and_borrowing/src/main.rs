fn main() {
// References
   let m1: String = String::from("Hello");
   let m2: String = String::from("World");
    great(&m1, &m2);
    //Implicit reference
    let s_len1: usize = m1.len();
    //Explicit reference
    let s_len2: usize = str::len(&m2);
    println!("This is slen1 {s_len1} and this is slen2 {s_len2}");


    //Dereference
    let mut x: Box<i32> = Box::new(4);
    let y: Box<&Box<i32>> = Box::new(&x);
    let z: i32 = ***y;
    println!("This is the y {y} and z {z}");
    *x += 2;
    println!("This is the new x = {}", x);
   let _s: String = format!("{} {}", m1, m2);
}

// References
fn great(g1: &str, g2: &str){
    println!("{} {}", g1, g2);
}
