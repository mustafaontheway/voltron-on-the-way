use std::collections::HashSet;

fn main() {

    let mut years: HashSet<u16> = HashSet::new();

    years.insert(2000);
    years.insert(2000);
    years.insert(2000);
    years.insert(2004);
    years.insert(2005);
    years.insert(2002);
    years.insert(2007);
    years.insert(1997);
    
    let _search_year1 = years.get(&2007).copied().unwrap_or(0);

    let _search_year2 = years.get(&3007).copied().unwrap_or(0);

}
