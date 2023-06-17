use varint::Varint;
fn main() {
    let u = 1_000_000_000_000_000;
    println!("{}", u);
    let varint = Varint::<4>::from(u);
    println!("{:?}", varint);
    println!("{}", varint.u128());
    println!("{}", Varint::<4>::floor(u));
}
