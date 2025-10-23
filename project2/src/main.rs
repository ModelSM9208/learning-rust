fn alpha(){
    let _x: &'static str = "two";
    let y: i32 = 2;
    let z: i32 = 2;
    assert_eq!(y,z);
    println!("y:{} equals z:{}",y,z)
}

fn beta(){
    let x: &'static str = "One";
    let y: &'static str = "Two";
    let z: &'static str = "Three";
    print!("{} something's got to give \n{} something's got to give \n{} something's got to give\n",x,y,z)
}

fn gamma(){
    let mut number: i32 = 1;
    println!("before delta {}",number);
    delta(&mut number);
    println!("after delta {}",number)
}
fn delta(number: &mut i32){
    *number += 2;
}

fn main() {
    alpha();
    beta();
    gamma();
}
