use std::fs::File; // fs -> file system

fn main() {

    let file_content = File::open("../dummy.txt");

    println!("{:?}", file_content);
}

//Ok(File { handle: 0xa8, path: "\\\\?\\D:\\_rust\\rust-again\\hi\\dummy.txt" })
