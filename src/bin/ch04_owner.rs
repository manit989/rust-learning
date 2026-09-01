fn main() {
    let mut v = vec![1, 2, 3];
    let last_item = v.last().unwrap();

    println!("{}", last_item);

    v.pop();
}
