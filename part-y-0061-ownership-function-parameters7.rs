fn main() {

    let mut g = greet();

    greet_mustafa(&mut g);
    
    g.push_str("and");

    greet_aykan(&mut g);

    g.push_str("and");

    greet_aybuke(&mut g);

    g.push('.');

    print_greet(&g); // Hello Mustafa and Aykan and Aybüke.

}

fn greet() -> String {

    String::from("Hello")
}

fn greet_mustafa(s: &mut String) {

    s.push_str(" Mustafa ")
}

fn greet_aykan(s: &mut String) {

    s.push_str(" Aykan ");
}

fn  greet_aybuke(s: &mut String) {

    s.push_str(" Aybüke");
}

fn print_greet(s: &String) {

    println!("{}", s)
}
// cargo run main.rs
