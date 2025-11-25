mod sort;

// use sort::bubble_sort;
// use sort::selection_sort;
use sort::insertion_sort;
fn main() {
    let mut arr: Vec<usize> = vec![7, 4, 3, 5, 3, 2, 1, 10, 45, 30, 20];

    // bubble_sort(&mut arr);
    // selection_sort(&mut arr);
    insertion_sort(&mut arr);
    println!("This is the new arr {:?}", arr);
}

