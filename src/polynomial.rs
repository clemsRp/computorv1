
pub struct Polynomial {
	coeffs: Vec<Fraction>,
}

impl Polynomial {
	fn normalized(raw_coeffs: Vec<Fraction>) -> Self {
		// Handle empty coeffs
		if raw_coeffs.len() == 0 {
			Polynomial{coeffs: raw_coeffs}
		}

		// Get last valid coeff
		let last_index = raw_coeffs.len() - 1;
		while last_index >= 0 && raw_coeffs[last_index].is_zero() {
			last_index -= 1;
		}

		// Get valid coeffs
		let mut i = 0;
		let mut coeffs: Vec<Fraction> = Vec::new();
		while i <= last_index {
			coeffs.push(raw_coeffs[i]);
			i += 1;
		}

		Polynomial{coeffs: coeffs}
	}

	pub fn new(raw_coeffs: Vec<Fraction>) -> Self {
		normalized(raw_coeffs)
	}

	fn get_coeffs(&self) -> &Vec<Fraction> {
		&self.coeffs
	}

	pub fn constant(value: Fraction) -> Self {
		normalized(vec![value])
	}

	pub fn x(coeff: Fraction, power: usize) -> Self {
		let mut raw_coeffs = vec![Fraction::default();power];
		raw_coeffs.push(coeff);
		normalized(raw_coeffs)
	}

	pub fn reciprocal(&self) -> Self {
		let mut res: Vec<Fraction> = Vec::new();

		// Get fractions
		let mut coeffs: Vec<Fraction> = self.get_coeffs();

		// Get fractions reciprocal
		for i in 0..coeffs.len() {
			res[i] = coeffs[i].reciprocal();
		}

		normalized(res)
	}

	pub fn add(&self, other: &Polynomial) -> Self {
		let mut res: Vec<Fraction> = Vec::new();

		// Get fractions
		let left = self.get_coeffs();
		let right = other.get_coeffs();

		// Get res len
		let l_len = left.len();
		let r_len = right.len();
		let length = l_len.max(r_len);

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
		self.add(&other.reciprocal())
	}
}
