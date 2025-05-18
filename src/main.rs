use num_bigint::BigUint;

fn calculate_length(value: &BigUint) -> u32 {
  value.to_str_radix(10).len() as u32
}

fn main() {
  println!("{}", calculate_length(&BigUint::from(2735u64)));
}
