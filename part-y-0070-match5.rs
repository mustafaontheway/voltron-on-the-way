fn main() {

    let r1 = color_to_number1("white");

    let r2 = color_to_number2("black");
}

fn color_to_number1(color: &str) -> u8 {

    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn color_to_number2(color: &str) -> u8 {

    match color {
      
        "red"=> 1,

        "green" => 2,

        "blue"=> 3,

        _ => 0
    }
}



//cargo run main.rs
