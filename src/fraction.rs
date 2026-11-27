
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::config::get_config;
use crate::error::{CalcError, CalcResult};
use crate::maths::{gcd, sqrt, from_f64};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fraction {
	num: i128,
	denum: i128,
}

impl Fraction {
	pub fn new(num: i128, denum: i128) -> Self {
		let mut new_num = num;
		let mut new_denum = denum;
		let divisor = gcd(new_num, new_denum);

		if denum < 0 {
			new_num *= -1;
			new_denum *= -1;
		}

        Fraction {
            num: new_num / divisor,
            denum: new_denum / divisor,
        }
	}

	pub fn try_new(num: i128, denum: i128) -> CalcResult<Self> {
		if denum == 0 {
			return Err(CalcError::new("Division by zero"));
		}
		let divisor = gcd(num, denum);
		let mut new_num = num / divisor;
		let mut new_denum = denum / divisor;

		if new_denum < 0 {
			new_num = new_num.checked_neg().ok_or_else(|| CalcError::new("Arithmetic overflow"))?;
			new_denum = new_denum.checked_neg().ok_or_else(|| CalcError::new("Arithmetic overflow"))?;
		}
		if new_num == i128::MIN {
			return Err(CalcError::new("Arithmetic overflow"));
		}

		Ok(Fraction {
			num: new_num,
			denum: new_denum,
		})
	}

	pub fn from_decimale(base_digits: String, frac_digits: String) -> CalcResult<Self> {
		let power: u32 = frac_digits
		    .len()
		    .try_into()
		    .map_err(|_| CalcError::new("number too large"))?;
		let scale = 10_i128.checked_pow(power).
			ok_or_else(|| CalcError::new("Too many decimal digits"))?;

		let base_int: i128 = base_digits.parse().map_err(|_| CalcError::new("number too large"))?;
        let frac_int: i128 = if frac_digits.is_empty() {
            0
        } else {
            frac_digits.parse().map_err(|_| CalcError::new("number too large"))?
        };

        let scaled = base_int.checked_mul(scale).
			ok_or_else(|| CalcError::new("Number is too large"))?;
        let raw_num = scaled.checked_add(frac_int).
			ok_or_else(|| CalcError::new("Number is too large"))?;
        let raw_denum = scale;

		Self::try_new(raw_num, raw_denum)
	}

	pub fn decimal_digits(&self) -> Option<usize> {
	    let mut power: i128 = 1;
	    let mut digits = 0;
	    while power < self.denum {
	        power = power.checked_mul(10)?;
	        digits += 1;
	    }
	    if power == self.denum { Some(digits) } else { None }
	}

	pub fn one() -> Self {
		Self::new(1, 1)
	}

	pub fn reciprocal(&self) -> CalcResult<Self> {
		if self.num == 0 {
			return Err(CalcError::new("Division by zero"));
		}

		Self::try_new(self.get_denum(), self.get_num())
	}

	pub fn pow(&self, power: u32) -> CalcResult<Self> {
		let new_num = self.num.checked_pow(power).
			ok_or_else(|| CalcError::new("Arithmetic overflow"))?;
        let new_denum = self.denum.checked_pow(power).
			ok_or_else(|| CalcError::new("Arithmetic overflow"))?;

		Self::try_new(new_num, new_denum)
	}

	pub fn sqrt(&self) -> CalcResult<Fraction> {
	    from_f64(sqrt(self.num as f64 / self.denum as f64))
	}

	// Helpers
	pub fn get_num(&self) -> i128 {
		self.num
	}

	pub fn get_denum(&self) -> i128 {
		self.denum
	}

	pub fn is_pos(&self) -> bool {
		self.num > 0
	}

	pub fn is_neg(&self) -> bool {
		self.num < 0
	}

	pub fn is_zero(&self) -> bool {
		self.num == 0
	}

	pub fn is_integer(&self) -> bool {
		self.denum == 1
	}
}

// Define Fraction display
impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_integer() {
            return write!(f, "{}", self.num);
        }

        if let Some(digits) = self.decimal_digits() {
            let sign = if self.num < 0 { "-" } else { "" };
            let n = self.num.unsigned_abs();
            let denum = self.denum as u128;
            let int_part = n / denum;
            let decimals = n % denum;
            return write!(f, "{}{}.{:0width$}", sign, int_part, decimals, width = digits);
        }

		let config = get_config();
		if config.fraction {
			return write!(f, "{}/{}", self.num, self.denum);
		} else {
			let value = self.num as f64 / self.denum as f64;
			let text = format!("{:.6}", value);
			let text = text.trim_end_matches('0').trim_end_matches('.');
			return write!(f, "{}", text);
		}
    }
}

// Define Fraction default value
impl Default for Fraction {
    fn default() -> Self {
        Self::new(0, 1)
    }
}

// Define Fraction operations
impl Neg for Fraction {
	type Output = CalcResult<Self>;
    fn neg(self) -> Self::Output {
        let num = self.num.checked_neg().ok_or_else(|| CalcError::new("Arithmetic overflow"))?;
        Ok(Self { num, denum: self.denum })
    }
}

impl Add for Fraction {
	type Output = CalcResult<Self>;
    fn add(self, other: Self) -> Self::Output {
        let g = gcd(self.denum, other.denum);
        let left_scale = other.denum / g;
        let right_scale = self.denum / g;

        let a = self.num.checked_mul(left_scale).ok_or_else(|| CalcError::new("Arithmetic overflow"))?;
        let b = other.num.checked_mul(right_scale).ok_or_else(|| CalcError::new("Arithmetic overflow"))?;
        let new_num = a.checked_add(b).ok_or_else(|| CalcError::new("Arithmetic overflow"))?;
        let new_denum = self.denum.checked_mul(left_scale).ok_or_else(|| CalcError::new("Arithmetic overflow"))?;

        Self::try_new(new_num, new_denum)
    }
}

impl Sub for Fraction {
	type Output = CalcResult<Self>;
    fn sub(self, other: Self) -> Self::Output {
        self + (-other)?
    }
}

impl Mul for Fraction {
	type Output = CalcResult<Self>;
    fn mul(self, other: Self) -> Self::Output {
        let g1 = gcd(self.num, other.denum);
        let g2 = gcd(other.num, self.denum);

        let a = self.num / g1;
        let b = other.num / g2;
        let c = self.denum / g2;
        let d = other.denum / g1;

        let new_num = a.checked_mul(b).ok_or_else(|| CalcError::new("Arithmetic overflow"))?;
        let new_denum = c.checked_mul(d).ok_or_else(|| CalcError::new("Arithmetic overflow"))?;

        Self::try_new(new_num, new_denum)
    }
}

impl Div for Fraction {
	type Output = CalcResult<Self>;
    fn div(self, other: Self) -> Self::Output {
        self * other.reciprocal()?
    }
}
