use num_bigint::BigUint;

fn calculate_length(value: &BigUint) -> u32 {
  value.to_str_radix(10).len() as u32
}

struct Split {
  low: BigUint,
  high: BigUint
}

fn calculate_split(value: &BigUint) -> Split {
  let length: u32 = calculate_length(value);
  let divisor: BigUint = BigUint::from(10u8).pow(length / 2);
  Split { low: value % &divisor, high: value / &divisor }
}

fn main() {
  println!("{}", calculate_length(&BigUint::from(2735u64)));
}
