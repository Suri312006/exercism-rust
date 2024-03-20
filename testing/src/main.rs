use reverse_string;

fn main() {
    let x = String::from("uüu");
    let y = reverse_string::reverse(&x);

    print!("{y}");

}
