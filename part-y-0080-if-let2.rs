fn main() {

    let loss_year_2025 = CompanyYearlyResult::loss(-100000);

    if let CompanyYearlyResult::loss(l) = loss_year_2025 {

        println!("Year 2025 loss amount: ${l}")
    }

    // Year 2025 loss amount: $-100000
}


enum CompanyYearlyResult {

    profit(u32),
    neutral,
    loss(i32)
}
