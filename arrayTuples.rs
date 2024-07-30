fn main() {
    // Tuple
    let tuple = (1, "hello", 3.14);
    println!("Tuple: {:?}", tuple);

    // Slice
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..3];
    println!("Slice: {:?}", slice);

    // Struct
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("John"),
        age: 30,
    };
    println!("Person: {:?}", person);

    // Array
    let array = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
}