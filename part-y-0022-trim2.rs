fn main() {

    let r = trim_and_capitalize(" hi   ");

    println!("{}", r) // HI

}

fn trim_and_capitalize(s: &str) -> String {

    let inner_string = s.trim().to_ascii_uppercase();

    inner_string
}
