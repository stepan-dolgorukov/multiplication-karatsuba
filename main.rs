use anyhow::{Context, Result, bail};
use num_bigint::BigUint;
use std::cmp::max;
use std::ops::Mul;
use std::str::FromStr;
use std::time::{Duration, Instant};

fn calculate_length(value: &BigUint) -> u32 {
  value.to_str_radix(10).len() as u32
}

struct Split {
  low: BigUint,
  high: BigUint,
}

impl Split {
  fn new(value: &BigUint, length_part: u32) -> Split {
    let divisor = BigUint::from(10u8).pow(length_part);
    Split {
      low: value % &divisor,
      high: value / &divisor,
    }
  }
}

struct BigUintKaratsubaMultiplication(BigUint);

impl BigUintKaratsubaMultiplication {
  fn new(value: BigUint) -> Self {
    BigUintKaratsubaMultiplication(value)
  }
}

impl Mul<&BigUint> for &BigUintKaratsubaMultiplication {
  type Output = BigUintKaratsubaMultiplication;
  fn mul(self, multiplier: &BigUint) -> Self::Output {
    let length_left: u32 = calculate_length(&self.0);
    let length_right: u32 = calculate_length(multiplier);

    if (length_left <= 4) && (length_right <= 4) {
      // здесь должно вызываться вычисление произведения столбиком
      return BigUintKaratsubaMultiplication(&self.0 * multiplier);
    }

    let half_maximum_length = max(length_left, length_right) / 2;
    let left_split = Split::new(&self.0, half_maximum_length);
    let right_split = Split::new(multiplier, half_maximum_length);

    /* left_split = (u_1, u_0)
    right_split = (v_1, v_0) */

    let multiplication_high_parts =
      &BigUintKaratsubaMultiplication(left_split.high.clone()) * &right_split.high;
    let multiplication_low_parts =
      &BigUintKaratsubaMultiplication(left_split.low.clone()) * &right_split.low;

    /* multiplication_high_parts = u_1 * v_1
    multiplication_low_parts = u_0 * v_0 */

    let multiplication_sums_low_high =
      &BigUintKaratsubaMultiplication(&left_split.high + &left_split.low)
        * &(&right_split.high + &right_split.low);

    /* multiplication_sums_low_high = (u_1 + u_0) * (v_1 + v_0) */

    let p = BigUint::from(10u8).pow(half_maximum_length);

    BigUintKaratsubaMultiplication(
      &multiplication_high_parts.0 * p.pow(2)
        + (&BigUintKaratsubaMultiplication(
          (multiplication_sums_low_high.0 - multiplication_high_parts.0)
            - multiplication_low_parts.0.clone(),
        ) * &p)
          .0
        + multiplication_low_parts.0,
    )

    /* product = multiplication_high_parts * (p**2) +
    (multiplication_sums_low_high - multiplication_high_parts - multiplication_low_parts) * p +
    multiplication_low_parts */
  }
}

fn read_biguint_from_stdin() -> Result<BigUint> {
  let mut input = String::new();

  match std::io::stdin().read_line(&mut input) {
    Ok(size_input) => {
      if size_input < 2 {
        bail!("a size of the input equals to 0");
      }

      BigUint::from_str(&mut input.trim())
        .context("fail to represent input as value of the type \"BigUint\"")
    }

    Err(_) => {
      bail!("fail to get input from \"stdin\"");
    }
  }
}

fn main() -> Result<()> {
  let operand_left = BigUintKaratsubaMultiplication::new(read_biguint_from_stdin()?);
  let operand_right: BigUint = read_biguint_from_stdin()?;
  let instant: Instant = Instant::now();
  let product = &operand_left * &operand_right;
  let duration_calculation: Duration = instant.elapsed();

  println!("{} * {} = {}", operand_left.0, operand_right, product.0);
  println!("{} ns", duration_calculation.as_nanos());
  Ok(())
}
