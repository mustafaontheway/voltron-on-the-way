use std::ops::{Range};

fn main() {

    let mut months: Range<u8>= 1..5;

    println!("Month 1: {:?}", months.next());

    println!("Month 2: {:?}", months.next());

    println!("Month 3: {:?}", months.next());

    println!("Month 4: {:?}", months.next());

    println!("Month 5: {:?}", months.next());

    println!("Month 6: {:?}", months.next());
}

// Month 1: Some(1)
// Month 2: Some(2)
// Month 3: Some(3)
// Month 4: Some(4)
// Month 5: None
// Month 6: None
