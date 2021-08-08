use safe_attr::safe;

union Num {
    f: f32,
    u: u32,
}

#[safe]
fn main() {
    let x = Num { f: 2.3 };
    println!("{}", x.u);
    let y = x.f;
    println!("{}", y);
}