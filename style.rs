fn main(){
    let a = add(20,22);
    println!("a = {}",a);
}
pub fn add(i:i32, j:i32) -> i32{
    i+j
}