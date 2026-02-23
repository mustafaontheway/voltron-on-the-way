fn main() {
  
    let mut year: u16 = 2025;

    let result1 = update_year_(year);

    println!("{:?}", year); // 2025

    let result2 = update_year(&mut year);

    println!("{:?}", year); // 2026
}

fn update_year(y: &mut u16) {

    *y += 1
}

fn update_year_(mut _y: u16) {

   _y += 1
}
