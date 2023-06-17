use varint::from_str;
use varint::to_string;
use varint::Varint;
fn main() {
    let varint = Varint::<2>::from(12001250521);
    println!("{:?}", varint);
    println!("{:?}", varint.u128());
    println!("{:?}", Varint::<2>::floor(12001250521));
    println!("{}", to_string::<18>(120));
    println!("{}", to_string::<18>(121209651062100));
    println!("{}", from_str::<18>("0.1121209651062100").unwrap());
}
