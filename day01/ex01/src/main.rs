fn main() {
    let a: u32 = 6;
    let b: u32 = 3;

    let add: u32 = a + b;
    let sub: u32 = a - b;
    let avg: f64 = (f64::from(a) + f64::from(b)) / 2.0;


    println!("{} + {} = {}", a, b, add);
    println!("{} + {} = {}", a, b, add);
    println!("{} - {} = {}", a, b, sub);
    println!("{} と {} の平均値：{}", a, b, avg);
}
