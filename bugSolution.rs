fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Safe way to access elements:
    match vec.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("Index out of bounds"),
    }

    // Alternative using if-let
    if let Some(third) = vec.get(2) {
        println!("The third element is {}", third);
    } else {
        println!("Index out of bounds");
    }
} 