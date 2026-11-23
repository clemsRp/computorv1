use crate::fraction::Fraction;

#[derive(Debug, Clone)]
pub struct Polynomial {
	coeffs: Vec<Fraction>,
}

impl Polynomial {
	fn normalized(raw_coeffs: Vec<Fraction>) -> Self {
		// Get last valid coeff
		let mut last_index = raw_coeffs.len();
		while last_index - 1 > 0 && raw_coeffs[last_index-1].is_zero() {
			last_index -= 1;
		}

		// Handle empty coeffs
		if raw_coeffs.len() == 0 {
			return Polynomial{coeffs: raw_coeffs};
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

	pub fn new(raw_coeffs: Vec<Fraction>) -> Self {
		Self::normalized(raw_coeffs)
	}

	fn get_coeffs(&self) -> &Vec<Fraction> {
		&self.coeffs
	}

	pub fn constant(value: Fraction) -> Self {
		Self::normalized(vec![value])
	}

	pub fn is_constant(&self) -> bool {
		self.get_coeffs().len() <= 1
	}

	pub fn x(coeff: Fraction, power: usize) -> Self {
		let mut raw_coeffs = vec![Fraction::default();power];
		raw_coeffs.push(coeff);
		Self::normalized(raw_coeffs)
	}

	pub fn negative(&self) -> Self {
		// Get fractions
		let coeffs: &Vec<Fraction> = self.get_coeffs();
		
		let mut res: Vec<Fraction> = vec![Fraction::default();coeffs.len()];

		// Get fractions reciprocal
		for i in 0..coeffs.len() {
			res[i] = coeffs[i].negative();
		}

		Self::normalized(res)
	}

	pub fn reciprocal(&self) -> Self {
		// Get fractions
		let coeffs: &Vec<Fraction> = self.get_coeffs();
		
		let mut res: Vec<Fraction> = vec![Fraction::default();coeffs.len()];

		// Get fractions reciprocal
		for i in 0..coeffs.len() {
			res[i] = coeffs[i].reciprocal();
		}

		Self::normalized(res)
	}

	pub fn add(&self, other: &Polynomial) -> Self {
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
				temp = temp.add(&left[i]);
			}
			if i < r_len {
				temp = temp.add(&right[i]);
			}

			res[i] = temp;
		}

		Self::normalized(res)
	}

	pub fn sub(&self, other: &Polynomial) -> Self {
		self.add(&other.negative())
	}

	pub fn mul(&self, other: &Polynomial) -> Self {
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
				res[i+j] = res[i+j].add(&l_frac.mul(&r_frac));
			}
		}

		Self::normalized(res)
	}

	pub fn div(&self, other: &Polynomial) -> Polynomial {
		// if !other.is_constant() {
		// 	TODO
		// }
		self.mul(&other.reciprocal())
	}

	pub fn pow(&self, power: u32) -> Self {
		let mut res = Self::constant(Fraction::one());
		let mut base = self.clone();
		let mut exp = power;

		while exp > 0 {
			if exp % 2 == 1 {
				res = res.mul(&base);
			}

			exp /= 2;

			if exp > 0 {
				base = base.mul(&base);
			}
		}

		res
	}

	pub fn print(&self) {
		let coeffs = self.get_coeffs();

		for (pow, coeff) in coeffs.iter().enumerate() {
			let mut sign = "";
			if !coeff.is_neg() && pow != 0 {
				sign = "+";
			}

			print!("{}{}X^{} ", sign, coeff.get_number(), pow);
		}
		println!();
	}
}
