fn main() {
    let a: i32 = 18;
    let b: char = 'A';

    // tuple
    let t = (a, b);

    // pattern matching
    let (c, d) = t;
    println!("c={}, d={}", c, d);

    println!("t.0={}", t.0);
    println!("t.1={}", t.1);
    
    let (result, is_overflow)  = a.overflowing_add(10);
    println!("result={}, is_overflow={}", result, is_overflow);

    // slice
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[0..3]; // range
    println!("slice[0]={}, len={}", slice[0], slice.len());
    let slice = &arr[3..5];
    println!("slice[0]={}, len={}", slice[0], slice.len());
    println!("{}", slice.len());
    println!("{}", slice.is_empty());

    let slice = &mut arr[..];
    slice[0] = 100000;
    println!("arr[0]={}", arr[0]);

    let a: i8 = -10;
    let b = a as u8;
    println!("a={} b={}", a, b);

    unsafe {
        let a = [0u8, 1u8, 0u8, 0u8];
        let b: u32 = mem::transmute(a);
        println!("{}", b);
    }
}
