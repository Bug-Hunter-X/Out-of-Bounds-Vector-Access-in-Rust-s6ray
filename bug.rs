fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let third = vec[2]; //This will panic because the index is out of bounds
    println!("The third element is {}", third);
}