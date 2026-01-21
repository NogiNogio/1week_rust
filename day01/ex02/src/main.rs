fn main() {
    let mut a1=2;  let mut b1=2;  let mut c1=2;  let mut d1=2;
    let mut a2=2;  let mut b2=2;  let mut c2=2;  let mut d2=2;

    a1 = a1 + 1;
    b1 = b1 - 1;
    c1 = c1 * 2;
    d1 = d1 / 2;

    a2 += 1;
    b2 += 1;
    c2 *= 2;
    d2 /= 2;

    println!("a1={a1} b1={b1} c1={c1} d1={d1}");
    println!("a2={a2} b2={b2} c2={c2} d2={d2}");
}
