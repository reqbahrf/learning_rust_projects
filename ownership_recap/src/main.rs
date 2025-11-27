// type Document = Vec<String>;

// fn new_document(words: Vec<String>) -> Document {
//     words
// }

// fn add_word(this: &mut Document, word: String){
//     this.push(word);
// }

// fn get_words(this: &Document) -> &[String] {
//     this.as_slice()
// }



fn main() {
    // let words = vec!["Hello".to_string()];
    // let d = new_document(words);

    // let words_copy = get_words(&d).to_vec();
    // let mut d2 = new_document(words_copy);
    // add_word(&mut d2, "World".to_string());

    // let mut v = Vec::from([2, 2, 4, 4]);
    let mut s = String::from("Hello");

    let s_ref = &mut s;
    // let v_ref:&mut [i32]  = &mut *v;
    
    // v_ref[3] = 5;
    // println!("{v:?}");
    s_ref.push_str(" World");
    println!("{s}");
}
