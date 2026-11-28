
use crate::error::{CalcError, CalcResult};
use crate::fraction::Fraction;

pub const PI: f64 = 3.14159265358979323846;

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

pub fn to_f64(f: Fraction) -> f64 {
    f.get_num() as f64 / f.get_denum() as f64
}

pub fn cbrt(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    if x < 0.0 {
        return -cbrt(-x);
    }
    let mut guess = if x > 1.0 { x } else { 1.0 };
    for _ in 0..1000 {
        let next = (2.0 * guess + x / (guess * guess)) / 3.0;
        if abs(next - guess) <= guess * 1e-15 {
            return next;
        }
        guess = next;
    }
    guess
}

pub fn cos(x: f64) -> f64 {
    let two_pi = 2.0 * PI;
    let mut x = x;
    while x > PI  { x -= two_pi; }
    while x < -PI { x += two_pi; }

    let x2 = x * x;
    let mut term = 1.0;
    let mut sum = 1.0;
    for n in 1..30 {
        term *= -x2 / ((2 * n - 1) as f64 * (2 * n) as f64);
        sum += term;
    }
    sum
}

pub fn acos(x: f64) -> f64 {
    let x = if x > 1.0 { 1.0 } else if x < -1.0 { -1.0 } else { x };
    let mut low = 0.0;
    let mut high = PI;
    for _ in 0..100 {
        let mid = (low + high) / 2.0;
        if cos(mid) > x { low = mid } else { high = mid }
    }
    (low + high) / 2.0
}
