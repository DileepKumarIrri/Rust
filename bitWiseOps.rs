fn main() {
    let a: u8 = 5; // Binary: 00000101
    let b: u8 = 3; // Binary: 00000011

    // Bitwise AND
    let bitwise_and = a & b; // Binary: 00000001
    println!("Bitwise AND: {}", bitwise_and);

    // Bitwise OR
    let bitwise_or = a | b; // Binary: 00000111
    println!("Bitwise OR: {}", bitwise_or);

    // Bitwise XOR
    let bitwise_xor = a ^ b; // Binary: 00000110
    println!("Bitwise XOR: {}", bitwise_xor);

    // Bitwise NOT
    let bitwise_not = !a; // Binary: 11111010
    println!("Bitwise NOT: {}", bitwise_not);

    // Logical AND
    let logical_and = a > 0 && b > 0;
    println!("Logical AND: {}", logical_and);

    // Logical OR
    let logical_or = a > 0 || b > 0;
    println!("Logical OR: {}", logical_or);

    // Logical NOT
    let logical_not = !(a > 0);
    println!("Logical NOT: {}", logical_not);
}