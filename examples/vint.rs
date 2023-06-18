use vint::vint;
use vint::Vint;
fn main() {
    let a = vint![100, 2];
    println!("{}", a);
    println!("{:?}", a);
    let vec = bincode::serialize(&a).unwrap();
    let b = bincode::deserialize::<Vint<2>>(&vec);
    println!("{:?}", b);
}
