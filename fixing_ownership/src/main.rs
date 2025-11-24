fn main() {
    let mut v: Vec<i32> = vec![1,2,3,5,6,3,6,3,4];

    multiple_in_place(&mut v);

    println!("New v {:?}", v);

    let mut n: i32 = 0;
    let a: &mut i32 = &mut n;

    println!("the value of a = {} and b", a);
    let b: &mut i32 = a;
    *b += 2;
    println!("the value of b = {}", b);

    // let mut point = [0, 1];
    // let mut x = point[0];
    // let y = &mut point[1];

    // x += 1;
    // *y += 1;

    // println!("{} {}", point[0], point[1]);

}

fn multiple_in_place(v: &mut Vec<i32>) {
    for n in v {
        *n *= 2
    };
}
