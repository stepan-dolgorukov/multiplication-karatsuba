use anyhow::{Context, Result, bail};
use num_bigint::BigUint;
use num_traits::Zero;
use std::cmp::max;
use std::str::FromStr;
use std::time::{Duration, Instant};

fn calculate_length(value: &BigUint) -> u32 {
  value.to_str_radix(10).len() as u32
}

struct Split {
  low: BigUint,
  high: BigUint,
}

fn calculate_split(value: &BigUint, length: u32) -> Split {
  let divisor: BigUint = BigUint::from(10u8).pow(length);
  Split {
    low: value % &divisor,
    high: value / &divisor,
  }
}

fn calculate_product(left: &BigUint, right: &BigUint) -> BigUint {
  // println!("{} {}", left, right);

  if left.is_zero() || right.is_zero() {
    return BigUint::zero();
  }

  let length_left: u32 = calculate_length(&left);
  let length_right: u32 = calculate_length(&right);

  if (length_left <= 4) && (length_right <= 4) {
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
  let c: BigUint = calculate_product(
    &(&split_left.high + &split_left.low),
    &(&split_right.high + &split_right.low),
  );

  // println!("a={}, b={}, c={}", a, b, c);

  let p: BigUint = BigUint::from(10u8).pow(n);
  &a * p.pow(2) + calculate_product(&((&c - &a) - &b), &p) + &b
}

fn read_biguint_from_stdin() -> Result<BigUint> {
  let mut input = String::new();

  match std::io::stdin().read_line(&mut input) {
    Ok(size_input) => {
      if size_input < 2 {
        bail!("a size of the input equals to 0");
      }

      BigUint::from_str(&mut input.trim()).context("fail to represent input as value of the type biguint")
    }

    Err(_) => {
      bail!("fail to get input from \"stdin\"");
    }
  }
}

fn main() -> Result<()> {
  let operand_left: BigUint = read_biguint_from_stdin()?;
  let operand_right: BigUint = read_biguint_from_stdin()?;
  let instant: Instant = Instant::now();
  let product: BigUint = calculate_product(&operand_left, &operand_right);
  let duration_calculation: Duration = instant.elapsed();

  println!("{} * {} = {}", operand_left, operand_right, product);
  println!("{} ns", duration_calculation.as_nanos());
  Ok(())
}
