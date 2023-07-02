/** Calculates the integer based on the operation given */
pub fn calculate(a: i8, b: i8, operatorString : &str) -> Result<i8, &str> {
    let op = giveOperation(operatorString);
    
    match op {
        Ok(operator) => return Ok(calculator(a, b, operator)),
        Err(err) => return Err(err)
    }
}

/** Gives the operation function based on the input */
fn giveOperation(operatingInput: &str ) -> Result<fn(x: i8, y: i8) -> i8, &str> {
    match operatingInput {
        "+" => return Ok(add),
        "-" => return Ok(subtract),
        "*" => return Ok(multiply),
        "/" => return Ok(divide),
        "%" => return Ok(modulo),
        "^" => return Ok(power),
        default => Err("The operation given is not supported"),

    }
}
/** General calculator  */
fn calculator (a: i8, b: i8, operator: fn(a: i8, b: i8) -> i8) -> i8{
    return operator(a, b);
}

/** Add function */
fn add(a: i8, b: i8) -> i8{
    return a + b;
}

/** Subtract function */
fn subtract(a: i8, b: i8) -> i8 {
    return a - b;
}

/** Multiply function */
fn multiply(a: i8, b: i8) -> i8 {
    return a * b;
}
/** Divide function */
fn divide(a: i8, b: i8) -> i8 {
    return a / b;
}

/** Modulo function */
fn modulo(a: i8, b: i8) -> i8 {
    return a % b;
}
/** Power function */ 
fn power(a: i8, b: i8) -> i8 {
    return a.pow(b as u32);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(1, 2, "+"), Ok(3));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(5, 2, "-"), Ok(3));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(4, 2, "*"), Ok(8));
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(10, 2, "/"), Ok(5));
    }

    #[test]
    fn test_modulo() {
        assert_eq!(calculate(11, 3, "%"), Ok(2));
    }

    #[test]
    fn test_power() {
        assert_eq!(calculate(2, 3, "^"), Ok(8));
    }

    #[test]
    fn test_unsupported_operation() {
        assert_eq!(calculate(5, 2, "&"), Err("The operation given is not supported"));
    }

    // Add a test to ensure correct behavior when division by zero is attempted
    #[test]
    #[should_panic]
    fn test_division_by_zero() {
        calculate(10, 0, "/").unwrap();
    }
}
