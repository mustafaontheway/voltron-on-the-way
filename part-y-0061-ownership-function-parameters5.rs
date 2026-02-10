fn main() {

    let content = String::from("lorem ipsum lorem ipsum lorem ipsumlorem ipsumlorem ipsumlorem ipsumlorem ipsumlorem ipsumlorem ipsumlorem ipsumlorem ipsum");

    println!("{:?}", calculate_len(&content)) // 123
}

fn calculate_len(c: &str) -> usize {

    c.len()
}
