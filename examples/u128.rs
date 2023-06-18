use varint::vint;
use varint::Varint;
fn main() {
    let a = vint![2u32, 2];
    println!("{:?}", a);
    println!("{}", a);
    let u = 1_000_000_000_000_000;
    println!("{}", u);
    let varint = Varint::<4>::from(u);
    println!("{:?}", varint);
    println!("{}", varint.u128());
    println!("{}", Varint::<4>::floor(u));
    let varint = Varint([1, 1]);
    let vec = bincode::serialize(&varint).unwrap();
    assert_eq!(vec.len(), 2);
    assert_eq!(varint.0.to_vec(), vec);
    let varint_2 = bincode::deserialize::<Varint<2>>(&vec);
    println!("{:?}", varint_2);
}
