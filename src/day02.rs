fn main() {
    println!("Hello, world 2!");

    let p = std::env::current_dir().unwrap();
    let s = p.to_str().unwrap();
    println!("You are at: {}", s);

    let mut v= Vec::new();
    v.push(32);
    v.push(64);

    let v2: Vec<i32> = v.iter().map(|v| v + 32).collect();
}
