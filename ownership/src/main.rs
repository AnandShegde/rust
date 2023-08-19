fn main() {

    {
        // s is not valid here because it’s not yet declared
        let s: String = String::from("hello");
        // s is valid here
        let slice = &s[0..2];
        arvind();
        println!("{}", &s);
        // let len: usize = get_len(&s);
        println!("{}", s);
        // println!("{}", len);
    } // s is deleted bcz it goes out of scope

    // // Slices
    // let s: String = String::from("anand world");
    // let word: &str = first_word(&s);
    // // s.clear();
    // println!("{}", word);
    


}

// fn get_len(s: &String) -> usize {
//     s.len()
// }

fn arvind(s: String){
    println!("{}", s);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // println!("{} {}", i, item);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[0..s.len()]
}


