fn main() {
    let country = String::from("Austria"); // country is not mutable, but we are going to print Austria-Hungary. How?
    adds_hungary(country);
}

fn adds_hungary(mut country: String) { // Here's how: adds_hungary takes the String and declares it mutable!
    country.push_str("-Hungary");
    println!("{}", country);
}
