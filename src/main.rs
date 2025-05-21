use num_bigint::BigUint;
use std::cmp::max;
use std::error::Error;
use num_traits::Zero;
use std::io::Read;
use std::str::FromStr;

fn calculate_length(value: &BigUint) -> u32 {
  value.to_str_radix(10).len() as u32
}

struct Split {
  low: BigUint,
  high: BigUint
}

fn calculate_split(value: &BigUint, length: u32) -> Split {
  let divisor: BigUint = BigUint::from(10u8).pow(length);
  Split { low: value % &divisor, high: value / &divisor }
}

fn calculate_product(left: &BigUint, right: &BigUint) -> BigUint {
  // println!("{} {}", left, right);

  if left.is_zero() || right.is_zero() {
    return BigUint::zero();
  }

  let length_left: u32 = calculate_length(&left);
  let length_right: u32 = calculate_length(&right);

  if (length_left <= 2) && (length_right <= 2) {
    // здесь должно вызываться вычисление произведения столбиком
    return left * right;
  }

  let n = max(length_left, length_right) / 2;
  let split_left: Split = calculate_split(&left, n);
  let split_right: Split = calculate_split(&right, n);

  // println!("{} {} {} {}", split_right.high, split_right.low, split_left.high, split_left.low);
  // a = u_1 * v_1
  let a: BigUint = calculate_product(&split_left.high, &split_right.high);

  // b = u_0 * v_0
  let b: BigUint = calculate_product(&split_left.low, &split_right.low);

  // c = (u_1 + u_0) * (v_1 + v_0)
  let c: BigUint = calculate_product(&(&split_left.high + &split_left.low),
                                     &(&split_right.high + &split_right.low));

  // println!("a={}, b={}, c={}", a, b, c);

  let p: BigUint = BigUint::from(10u8).pow(n);
  &a * p.pow(2) + calculate_product(&((&c - &a) - &b), &p) + &b
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut input = String::new();

  match std::io::stdin().read_to_string(&mut input) {
    Ok(_) => {}

    Err(_) => {
      return Err("fail to read from \"stdin\"".into());
    }
  };

  let mut operands = input.split_whitespace().map(
    |input: &str | BigUint::from_str(input).expect("fail to read number")
  );

  let operand_left: BigUint = operands.next().expect("fail to get a left operand");
  let operand_right: BigUint = operands.next().expect("fail to get a right operand");
  let product: BigUint = calculate_product(&operand_left, &operand_right);

  println!("{} * {} = {}", operand_left, operand_right, product);
  Ok(())
}
