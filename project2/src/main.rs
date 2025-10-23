fn beta(){
    let _x: &'static str = "two";
    let y: i32 = 2;
    let z: i32 = 2;
    assert_eq!(y,z);
    println!("y:{} equals z:{}",y,z)
}

fn main() {
    beta();
}
