fn main() {
    let mut x = 4;
    println!("x={} x+1={}", x, x + 1);
    x = x + 1;
    {
        let x = 2;
        println!("x={} x+1={}", x, x + 1);
    }
    let x = x + 1;
    println!("x={} x+1={}", x, x + 1);

    let x = "hello";
    println!("x={}", x);

    const MY_CONST: u32 = 60;
    println!("my const={}", MY_CONST);
}
