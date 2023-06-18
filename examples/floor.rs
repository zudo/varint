use varint::floor;
use varint::Varint;
fn main() {
    let a = floor![100, 1];
    println!("{}", a);
    let b = floor![100, 2];
    println!("{}", b);
}
