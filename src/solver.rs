
use crate::error::CalcResult;
use crate::equation::Equation;

pub fn get_equation_solutions(input: &str) -> CalcResult<Vec<String>> {
	let mut final_prints: Vec<String> = Vec::new();

	// Create equation
	let mut equation = Equation::new(input)?;
	final_prints.push(format!("[step] Raw form: {}", equation));

	// Simplified it
	let sub = equation.clone();
	let sub_equation = format!("{} - ({}) = {} - ({})", sub.get_left(), sub.get_right(), sub.get_right(), sub.get_right());
	final_prints.push(format!("[step] Subtract from both sides: {}", sub_equation));
	equation = equation.reduced()?;
	final_prints.push(format!("Reduced form: {}", equation));
	
	// Get solutions
	let solved = equation.solve()?;
	final_prints.extend(solved);

	Ok(final_prints)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{FormatConfig, set_config};

    fn setup() {
        set_config(FormatConfig { fraction: false, simple: true });
    }

    fn solve(input: &str) -> Vec<String> {
        setup();
        get_equation_solutions(input)
            .unwrap()
            .into_iter()
            .filter(|l| !l.starts_with("[step]"))
            .collect()
    }

    fn solve_err(input: &str) -> bool {
        setup();
        get_equation_solutions(input).is_err()
    }

    // Degree 0
    #[test]
    fn test_trivially_true() {
        let out = solve("0 = 0");
        assert!(out.iter().any(|l| l.contains("Any real number")), "{:?}", out);
    }

    #[test]
    fn test_trivially_false() {
        let out = solve("1 = 2");
        assert!(out.iter().any(|l| l.contains("No solution")), "{:?}", out);
    }

    // Degree 1
    #[test]
    fn test_degree_one_simple() {
        let out = solve("X = 0");
        assert!(out.iter().any(|l| l == "0"), "{:?}", out);
    }

    #[test]
    fn test_degree_one_positive() {
        let out = solve("X - 5 = 0");
        assert!(out.iter().any(|l| l == "5"), "{:?}", out);
    }

    #[test]
    fn test_degree_one_negative() {
        let out = solve("X + 3 = 0");
        assert!(out.iter().any(|l| l == "-3"), "{:?}", out);
    }

    #[test]
    fn test_degree_one_with_coeff() {
        let out = solve("2 * X - 4 = 0");
        assert!(out.iter().any(|l| l == "2"), "{:?}", out);
    }

    #[test]
    fn test_degree_one_fractional_solution() {
        setup();
        let out = get_equation_solutions("3 * X - 1 = 0")
            .unwrap()
            .into_iter()
            .filter(|l| !l.starts_with("[step]"))
            .collect::<Vec<_>>();

        let solution_line = out.iter()
            .find(|l| l.contains("0.333") || l.contains("1/3"))
            .expect("Expected fractional solution in output");
        assert!(!solution_line.is_empty());
    }

    // Degree 2
    #[test]
    fn test_degree_two_positive_discriminant() {
        let out = solve("X^2 - 5 * X + 6 = 0");
        assert!(out.iter().any(|l| l.contains("strictly positive")), "{:?}", out);
        assert!(out.iter().any(|l| l == "3"), "{:?}", out);
        assert!(out.iter().any(|l| l == "2"), "{:?}", out);
    }

    #[test]
    fn test_degree_two_zero_discriminant() {
        let out = solve("X^2 - 2 * X + 1 = 0");
        assert!(out.iter().any(|l| l.contains("null")), "{:?}", out);
        assert!(out.iter().any(|l| l == "1"), "{:?}", out);
    }

    #[test]
    fn test_degree_two_negative_discriminant() {
        let out = solve("X^2 + 1 = 0");
        assert!(out.iter().any(|l| l.contains("complex")), "{:?}", out);
    }

    #[test]
    fn test_degree_two_negative_discriminant_values() {
        set_config(FormatConfig { fraction: true, simple: true });
        let out = get_equation_solutions("X^2 + X + 1 = 0")
            .unwrap()
            .into_iter()
            .filter(|l| !l.starts_with("[step]") && !l.starts_with("Reduced") && !l.starts_with("Polynomial") && !l.starts_with("Discriminant"))
            .collect::<Vec<_>>();

        let solutions: Vec<_> = out.iter().filter(|l| l.ends_with('i') || l.contains("i/") || l.contains("i,")).collect();
        assert_eq!(solutions.len(), 2, "Expected 2 complex solutions, got: {:?}", out);
    }

    // Degree 3
    #[test]
    fn test_degree_three_positive_discriminant() {
        let out = solve("X^3 - 6 * X^2 + 11 * X - 6 = 0");
        assert!(out.iter().any(|l| l.contains("strictly positive")), "{:?}", out);
        assert!(out.iter().any(|l| l == "1"), "{:?}", out);
        assert!(out.iter().any(|l| l == "2"), "{:?}", out);
        assert!(out.iter().any(|l| l == "3"), "{:?}", out);
    }

    #[test]
    fn test_degree_three_negative_discriminant() {
        let out = solve("X^3 + X + 1 = 0");
        assert!(out.iter().any(|l| l.contains("strictly negative")), "{:?}", out);
    }

    #[test]
    fn test_degree_three_zero_discriminant_triple() {
        let out = solve("X^3 - 3 * X^2 + 3 * X - 1 = 0");
        assert!(out.iter().any(|l| l.contains("triple")), "{:?}", out);
        assert!(out.iter().any(|l| l == "1"), "{:?}", out);
    }

    #[test]
    fn test_degree_three_zero_discriminant_double() {
        let out = solve("X^3 - 5 * X^2 + 8 * X - 4 = 0");
        assert!(out.iter().any(|l| l.contains("double")), "{:?}", out);
        assert!(out.iter().any(|l| l == "2"), "{:?}", out);
        assert!(out.iter().any(|l| l == "1"), "{:?}", out);
    }

    // Degree > 3
    #[test]
    fn test_degree_too_high() {
        let out = solve("X^4 = 0");
        assert!(out.iter().any(|l| l.contains("greater than 3")), "{:?}", out);
    }

    // Error cases
    #[test]
    fn test_missing_equal_sign() {
        assert!(solve_err("X + 1"));
    }

    #[test]
    fn test_empty_equation_err() {
        assert!(solve_err("="));
    }

    #[test]
    fn test_invalid_token() {
        assert!(solve_err("@ = 0"));
    }
}
