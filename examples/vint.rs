use varint::vint;
use varint::Varint;
fn main() {
    let a = vint![100, 2];
    println!("{}", a);
    println!("{:?}", a);
    let vec = bincode::serialize(&a).unwrap();
    let b = bincode::deserialize::<Varint<2>>(&vec);
    println!("{:?}", b);
}
