
#[derive(Debug, Clone, Copy)]
pub struct Fraction {
	num: i128,
	denum: i128,
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
	a = a.abs();
	b = b.abs();

	while b != 0 {
		let r = a % b;
		a = b;
		b = r;
	}

	a.max(1)
}

impl Fraction {
	pub fn new(num: i128, denum: i128) -> Self {
		let divisor = gcd(num, denum);

        Fraction {
            num: num / divisor,
            denum: denum / divisor,
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

	pub fn default() -> Self {
		Self::new(0, 1)
	}

	pub fn one() -> Self {
		Self::new(1, 1)
	}

	pub fn get_num(&self) -> i128 {
		self.num
	}

	pub fn get_denum(&self) -> i128 {
		self.denum
	}

	pub fn is_neg(&self) -> bool {
		self.num < 0
	}

	pub fn is_zero(&self) -> bool {
		self.num == 0
	}

	pub fn negative(&self) -> Self {
		Self{num: -self.get_num(), denum: self.get_denum()}
	}

	pub fn reciprocal(&self) -> Self {
		Self{num: self.get_denum(), denum: self.get_num()}
	}

	pub fn add(&self, other: &Fraction) -> Self {
		let new_num = self.num * other.get_denum() + other.get_num() * self.denum;
		let new_denum = self.denum * other.get_denum();

		Self::new(new_num, new_denum)
	}

	pub fn sub(&self, other: &Fraction) -> Self {
		self.add(&other.negative())
	}

	pub fn mul(&self, other: &Fraction) -> Self {
		let new_num = self.num * other.get_num();
		let new_denum = self.denum * other.get_denum();

		Self::new(new_num, new_denum)
	}

	pub fn div(&self, other: &Fraction) -> Self {
		self.mul(&other.reciprocal())
	}

	pub fn pow(&self, power: u32) -> Self {
		let new_num = self.num.pow(power);
		let new_denum = self.denum.pow(power);

		Self::new(new_num, new_denum)
	}

	pub fn get_number(&self) -> String {
		(self.num as f64 / self.denum as f64).to_string()
	}
}
