// ============================================================================
// Calculatrice simple : 2 nombres, 4 opérations (+ - * /)
// ============================================================================

use std::io::{self, Write};

// ----------------------------------------------------------------------------
// 1) Un enum : un type qui ne peut valoir QUE l'une de ces 4 valeurs
// ----------------------------------------------------------------------------
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

// ----------------------------------------------------------------------------
// 2) Lire une ligne tapée par l'utilisateur
// ----------------------------------------------------------------------------
fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // force l'affichage du prompt avant la saisie

    let mut line = String::new(); // `mut` : on va la remplir
    io::stdin().read_line(&mut line).unwrap(); // &mut = on prête la String modifiable
    line.trim().to_string() // trim() retire le "\n" et les espaces autour
}

// ----------------------------------------------------------------------------
// 3) Convertir du texte en nombre (peut échouer -> Result)
// ----------------------------------------------------------------------------
fn read_number(prompt: &str) -> Result<f64, String> {
    let text = read_line(prompt);
    text.parse::<f64>()
        .map_err(|_| format!("'{}' n'est pas un nombre valide", text))
}

// ----------------------------------------------------------------------------
// 4) Convertir le texte "+", "-", "*", "/" en Operation
// ----------------------------------------------------------------------------
fn read_operation(prompt: &str) -> Result<Operation, String> {
    let text = read_line(prompt);
    match text.as_str() {
        "+" => Ok(Operation::Add),
        "-" => Ok(Operation::Sub),
        "*" => Ok(Operation::Mul),
        "/" => Ok(Operation::Div),
        _ => Err(format!("'{}' n'est pas une opération (+ - * /)", text)),
    }
}

// ----------------------------------------------------------------------------
// 5) Le calcul. La division par zéro renvoie une erreur.
// ----------------------------------------------------------------------------
fn calculate(a: f64, op: &Operation, b: f64) -> Result<f64, String> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Sub => Ok(a - b),
        Operation::Mul => Ok(a * b),
        Operation::Div => {
            if b == 0.0 {
                Err("division par zéro impossible".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
}

// ----------------------------------------------------------------------------
// 6) Tout assembler. Le `?` propage l'erreur vers l'appelant.
// ----------------------------------------------------------------------------
fn run() -> Result<(), String> {
    let a = read_number("Premier nombre : ")?;
    let op = read_operation("Opération (+ - * /) : ")?;
    let b = read_number("Deuxième nombre : ")?;

    let result = calculate(a, &op, b)?;
    println!("Résultat : {}", result);
    Ok(()) // () = "rien à renvoyer", mais tout s'est bien passé
}

fn main() {
    if let Err(message) = run() {
        eprintln!("Erreur : {}", message);
        std::process::exit(1);
    }
}