use unicode_segmentation::UnicodeSegmentation;
// takes in a reference to a string input, returns a String struct
pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
        
    // collects everything as a vector
    let mut graphemed = input.graphemes(true).collect::<Vec<&str>>();

    //should be reversed now
    graphemed.reverse();

    for x in graphemed.iter() {
        // cant call this here since it just dont work like that
        reversed.push_str(*x)
    }

    // dude this seems so fucked lmfao

    // let reversed_word: String = String::from(reversed.iter().collect::<str>());

    return reversed;
}
