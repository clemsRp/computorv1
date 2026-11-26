
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::config::get_fraction_mode;
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

	pub fn from_decimale(base_digits: String, frac_digits: String) -> Self {
		let power: u32 = frac_digits.len().try_into().unwrap();
		let scale = 10_i128.pow(power);

		let base_int: i128 = base_digits.parse().unwrap();
        let frac_int: i128 = if frac_digits.is_empty() {
            0
        } else {
            frac_digits.parse().unwrap()
        };

        let raw_num = base_int * scale + frac_int;
        let raw_denum = scale;

		Self::new(raw_num, raw_denum)
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

	pub fn reciprocal(&self) -> Self {
		Self::new(self.get_denum(), self.get_num())
	}

	pub fn pow(&self, power: u32) -> Self {
		let new_num = self.num.pow(power);
		let new_denum = self.denum.pow(power);

		Self::new(new_num, new_denum)
	}

	pub fn sqrt(&self) -> Fraction {
	    let num_root = from_f64(sqrt(self.num as f64));
	    let den_root = from_f64(sqrt(self.denum as f64));
	    num_root / den_root
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
            let n = self.num.abs();
            let int_part = n / self.denum;
            let decimals = n % self.denum;
            return write!(f, "{}{}.{:0width$}", sign, int_part, decimals, width = digits);
        }

		let want_fraction = get_fraction_mode();
		if want_fraction {
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
	type Output = Self;
    fn neg(self) -> Self {
        Self{num: -self.get_num(), denum: self.get_denum()}
    }
}

impl Add for Fraction {
	type Output = Self;
    fn add(self, other: Self) -> Self {
        let new_num = self.num * other.get_denum() + other.get_num() * self.denum;
		let new_denum = self.denum * other.get_denum();

		Self::new(new_num, new_denum)
    }
}

impl Sub for Fraction {
	type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.add(-other)
    }
}

impl Mul for Fraction {
	type Output = Self;
    fn mul(self, other: Self) -> Self {
        let new_num = self.num * other.get_num();
		let new_denum = self.denum * other.get_denum();

		Self::new(new_num, new_denum)
    }
}

impl Div for Fraction {
	type Output = Self;
    fn div(self, other: Self) -> Self {
        self.mul(other.reciprocal())
    }
}
