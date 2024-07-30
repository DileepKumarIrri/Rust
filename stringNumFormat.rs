fn main() {
    let name = "Lokesh";
    let age = 20;
    let height = 1.75;

    // Format string
    let formatted_string = format!("My name is {} and I am {} years old.", name, age);
    println!("{}", formatted_string);

    // Format number
    let formatted_number = format!("{:.2}", height);
    println!("My height is {} meters.", formatted_number);
}