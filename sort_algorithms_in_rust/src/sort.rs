use std::usize;

pub fn bubble_sort(vec: &mut Vec<usize>) {
    let n: usize = vec.len();
    for i in 0..(n-1) {
       for j in 0..(n-1-i) {
        if vec[j] > vec[j + 1] {
          vec.swap(j, j + 1);
        }
       }
    }
    println!("This is the n of arr {n}")
}

pub fn selection_sort(vec: &mut Vec<usize>) {
   let n = vec.len();
   for i in 0..(n - 1){
    let mut min_index = i;
    let mut j = i + 1;
    while j < n {
      if vec[j] < vec[min_index] {
        min_index = j;
      }
      j += 1;
    }
    if min_index != i {
      vec.swap(i, min_index);
    }
   }
}

pub fn insertion_sort(vec: &mut Vec<usize>) {
  let n = vec.len();

  for i in 1..n {
    let temp = vec[i];
    let mut j  = i;

    while j > 0 && temp < vec[j - 1] {
      vec[j] = vec[j - 1];
      j -= 1
    }
    vec[j] = temp;
  }
}