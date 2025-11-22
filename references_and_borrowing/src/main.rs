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



   //Simultaneous Aliasing and Mutation
   let mut v: Vec<i32> = vec![1,2,3];
   let num: &i32 = &v[2];
   println!("This is the val of num {}", num);
   v.push(3);
   println!("This is the new array {:?}", v);

   println!("This is the array val at index 3 = {}", v[3]);

   //Mutable Reference
   let mut v2: Vec<i32> = vec![2,9,4,0];
   let num2:  &mut i32 = &mut v2[2];
   *num2 += 1;
   println!("Third element is {}", *num2);

   let first: &mut i32 = first( &mut v2);
   *first += 1;

   println!("This is the first element {}", v2[0]);
   println!("Vector is now {:?}", v2);


}

fn first(v_r: &mut Vec<i32>) -> &mut i32 {
    let s_ref: &mut i32 = &mut v_r[0];
    s_ref
}

// References
fn great(g1: &str, g2: &str){
    println!("{} {}", g1, g2);
}
