
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::error::{CalcError, CalcResult};
use crate::config::get_config;
use crate::fraction::Fraction;

#[derive(Debug, Clone)]
pub struct Polynomial {
	coeffs: Vec<Fraction>,
}

impl Polynomial {
	fn normalized(raw_coeffs: Vec<Fraction>) -> Self {
		// Handle empty coeffs
		if raw_coeffs.len() == 0 {
			return Polynomial{coeffs: raw_coeffs};
		}

		// Get last valid coeff
		let mut last_index = raw_coeffs.len();
		while last_index - 1 > 0 && raw_coeffs[last_index-1].is_zero() {
			last_index -= 1;
		}

		// Get valid coeffs
		let mut i = 0;
		let mut coeffs: Vec<Fraction> = Vec::new();
		while i <= last_index - 1 {
			coeffs.push(raw_coeffs[i].clone());
			i += 1;
		}

		Polynomial{coeffs: coeffs}
	}

	pub fn constant(value: Fraction) -> Self {
		Self::normalized(vec![value])
	}

	pub fn x() -> Self {
		Self::normalized(vec![Fraction::default(), Fraction::one()])
	}

	pub fn pow(&self, power: u32) -> CalcResult<Self> {
		let mut res = Self::constant(Fraction::one());
		let mut base = self.clone();
		let mut exp = power;

		while exp > 0 {
			if exp % 2 == 1 {
				res = (res * base.clone())?;
			}

			exp /= 2;

			if exp > 0 {
				base = (base.clone() * base.clone())?;
			}
		}

		Ok(res)
	}

	// Helpers
	pub fn get_coeffs(&self) -> &[Fraction] {
		&self.coeffs
	}

	pub fn is_zero(&self) -> bool {
		let coeffs = self.get_coeffs();
		coeffs.len() == 1 && coeffs[0].is_zero()
	}

	pub fn as_constant(&self) -> CalcResult<Fraction> {
		let coeffs = self.get_coeffs();
		if coeffs.len() == 1 {
			Ok(coeffs[0])
		} else {
			Err(CalcError::new("Polynomial is not a constant"))
		}
	}
}

// Define Polynomial display
impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
		let config = get_config();

        for (power, factor) in self.coeffs.iter().enumerate() {
            // Skip if factor is 0
            if factor.is_zero() && config.simple {
                continue;
            }

            // Calculate factor sign print
            let abs = if factor.is_neg() { (-*factor).unwrap_or(*factor) } else { *factor };
            let is_first = res.is_empty();
            if factor.is_neg() {
                res.push_str(if is_first { "-" } else { " - " });
            } else if !is_first {
                res.push_str(" + ");
            }

            // Calculate factor print
            if power == 0 || abs != Fraction::one() {
                res.push_str(&abs.to_string());
                if !config.simple {
                    res.push_str(" * ");
                }
            }

            // Calculate X power print
			if power == 1 && config.simple {
                res.push_str(&format!("X"));
			} else if power != 0 || !config.simple {
                res.push_str(&format!("X^{}", power));
			}
        }

		// Display 0 if result if null
        if res.is_empty() {
			if config.simple {
				res.push('0');
			} else {
				res.push_str("0 * X^0");
			}
        }

        write!(f, "{}", res)
    }
}

// Define Polynamial default value
impl Default for Polynomial {
    fn default() -> Self {
        Self::normalized(vec![Fraction::default()])
    }
}

// Define Polynaomial operations
impl Neg for Polynomial {
	type Output = CalcResult<Self>;
    fn neg(self) -> Self::Output {
        // Get fractions
		let coeffs = self.get_coeffs();
		
		let mut res: Vec<Fraction> = vec![Fraction::default();coeffs.len()];

		// Get fractions reciprocal
		for i in 0..coeffs.len() {
			res[i] = (-coeffs[i])?;
		}

		Ok(Self::normalized(res))
    }
}

impl Add for Polynomial {
	type Output = CalcResult<Self>;
    fn add(self, other: Self) -> Self::Output {
        // Get fractions
		let left = self.get_coeffs();
		let right = other.get_coeffs();
		
		// Get res len
		let l_len = left.len();
		let r_len = right.len();
		let length = l_len.max(r_len);
		
		let mut res: Vec<Fraction> = vec![Fraction::default();length];

		// Fill res
		for i in 0..length {
			let mut temp = Fraction::default();
			if i < l_len {
				temp = (temp + left[i])?;
			}
			if i < r_len {
				temp = (temp + right[i])?;
			}

			res[i] = temp;
		}

		Ok(Self::normalized(res))
    }
}

impl Sub for Polynomial {
	type Output = CalcResult<Self>;
    fn sub(self, other: Self) -> Self::Output {
        self + (-other)?
    }
}

impl Mul for Polynomial {
	type Output = CalcResult<Self>;
    fn mul(self, other: Self) -> Self::Output {
        // Get fractions
		let left = self.get_coeffs();
		let right = other.get_coeffs();
		
		// Get res len
		let l_len = left.len();
		let r_len = right.len();
		
		// Init result vector
		let mut res: Vec<Fraction> = vec![Fraction::default();l_len+r_len];

		// Execute double distribution
		for (i, l_frac) in left.iter().enumerate() {
			for (j, r_frac) in right.iter().enumerate() {
				res[i+j] = (res[i+j] + (*l_frac * *r_frac)?)?;
			}
		}

		Ok(Self::normalized(res))
    }
}

impl Div for Polynomial {
    type Output = CalcResult<Self>;

    fn div(self, other: Self) -> Self::Output {
        let Ok(c) = other.as_constant() else {
            return Err(CalcError::new("Can only divide polynomial by constants"));
        };
        if c.is_zero() {
            return Err(CalcError::new("Division by zero"));
        }

        self * Polynomial::constant(c.reciprocal()?)
    }
}
