fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; //Safe and recommended approach
    println!("v = {:?}", v);
}