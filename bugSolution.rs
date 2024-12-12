fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // Correct way to modify a vector element.
    println!("v: {:?}", v);

    // Alternative solution using iterators
    let mut v2 = vec![1, 2, 3];
    v2.iter_mut().next().unwrap() = &mut 10;
    println!("v2: {:?}", v2);
} 