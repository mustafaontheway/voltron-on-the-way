use std::io;

fn main() {

    let mut user_info = String::new();

    println!("Give us some details: ");

    io::stdin().read_line(&mut user_info).expect("Unexpected error! Try again later...");

    println!("Thank you. You sad: \"{}\"", user_info.trim());
}

/* 
My name is Mustafa. I'll come back later.
Thank you. You sad: "My name is Mustafa. I'll come back later."

Empty?
Thank you. You sad: ""
*/
