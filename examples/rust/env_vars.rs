fn main() {
    let my_var = std::env::var("MY_VAR").unwrap_or_default();
    let debug = std::env::var("DEBUG").unwrap_or_default();
    let greeting = std::env::var("GREETING").unwrap_or_default();
    println!("MY_VAR={my_var}");
    println!("DEBUG={debug}");
    println!("GREETING={greeting}");
}
