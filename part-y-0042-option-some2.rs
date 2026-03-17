fn main() {

    let sales: Option<u32> = Some(650000);

    //let sales: Option<u32> = None;

    let mut premium: f32 = match sales {
        Some(v) => v as f32 * 0.06,
        None => 0.0
    };

    println!("Premium: {premium}");

    premium += 630.0;

    println!("Premium plus: {premium}");
}

// Premium: 39000
// Premium plus: 39630



