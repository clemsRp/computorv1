
use crate::fraction::Fraction;

pub fn gcd(mut a: i128, mut b: i128) -> i128 {
	a = a.abs();
	b = b.abs();

	while b != 0 {
		let r = a % b;
		a = b;
		b = r;
	}

	a.max(1)
}

pub fn abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

pub fn sqrt(x: f64) -> f64 {
	if x <= 0.0 {
	    return 0.0;
	}
	let mut guess = if x > 1.0 { x } else { 1.0 };
	for _ in 0..1000 {
	    let next = (guess + x / guess) / 2.0;
	    if abs(next - guess) <= guess * 1e-15 {
	        return next;
	    }
	    guess = next;
	}
	guess
}

pub fn from_f64(x: f64) -> Fraction {
    let text = format!("{:.6}", x);
    let (int_part, frac_part) = text.split_once('.').unwrap();
    Fraction::from_decimale(int_part.to_string(), frac_part.to_string())
}
