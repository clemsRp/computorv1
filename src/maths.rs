
use crate::error::{CalcError, CalcResult};
use crate::fraction::Fraction;

pub fn gcd(a: i128, b: i128) -> i128 {
	let mut a = a.unsigned_abs();
	let mut b = b.unsigned_abs();

	while b != 0 {
		let r = a % b;
		a = b;
		b = r;
	}

	i128::try_from(a.max(1)).unwrap_or(1)
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

pub fn from_f64(x: f64) -> CalcResult<Fraction> {
    if !x.is_finite() {
        return Err(CalcError::new("Cannot convert a non-finite number"));
    }

    let text = format!("{:.6}", x);
    let Some((int_part, frac_part)) = text.split_once('.') else {
        return Err(CalcError::new("Failed to format float"));
    };

    Fraction::from_decimale(int_part.to_string(), frac_part.to_string())
}
