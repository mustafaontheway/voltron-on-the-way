fn main() {
    
    let mut vals = vec![0i8; 15];

    vals[5] = -100;

    println!("{:?}", vals); // [0, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    // unsafe

    //println!("{:?}", vals[97]) //panic!

    println!("{:?}", vals.get(97)); // None
}
