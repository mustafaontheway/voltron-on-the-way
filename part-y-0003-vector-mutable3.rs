fn main() {

    let mut odds: Vec<u8> = Vec::new();
    let mut evens: Vec<u8> = Vec::new();

    let mut counter: u8 = 0;

    loop {

        if counter % 2 == 0 {

            evens.push(counter);

        } else {
            
            odds.push(counter);
        }

        counter += 1;

        if counter == 30 {

            break;
        }
    }

    println!("Even numbers: {:?}", &evens);

    println!("Odd numbers: {:?}", &odds);

}

// Even numbers: [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28]
// Odd numbers: [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29]

