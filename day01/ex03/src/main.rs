fn main() {
    let j1: i32 = 3;
    let j2: i32 = 3;

    let d1: f64 = 1.23;
    let d2: f64 = 1.23;

    let i1: i32 = d1 as i32;
    let i2: i32 = d2 as i32;

    let e1: f64 = j1 as f64;
    let e2: f64 = j2 as f64;

    println!("d1 = {d1:.2} \t d2 = {d2:.2}");
    println!("i1 = {i1}    \t i2 = {i2}");
    println!("j1 = {j1}    \t j2 = {j2}");
    println!("e1 = {e1:.2} \t e2 = {e2:.2}");
}
