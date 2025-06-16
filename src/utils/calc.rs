pub fn calc(expr: &str) -> i64 {
    let postfix: String = infix_to_postfix(expr);
    return calc_postfix(&postfix);
}

fn precedence(op: char) -> i32 {
    match op {
        '*' | '/' => 2,
        '+' | '-' => 1,
        _ => 0,
    }
}

fn infix_to_postfix(expr: &str) -> String {
    let mut stack = Vec::new();
    let mut output = String::new();

    for ch in expr.chars() {
        if ch.is_whitespace() {
            continue;
        } else if ch.is_digit(10) {
            output.push(ch);
        } else if ch == '(' {
            stack.push('(');
        } else if ch == ')' {
            while let Some(top) = stack.pop() {
                if top == '(' {
                    break;
                } else {
                    output.push(top);
                }
            }
        } else if "-+*/".contains(ch) {
            while let Some(&top) = stack.last() {
                if top == '(' {
                    break;
                }

                let top_prec = precedence(top);
                let ch_prec = precedence(ch);

                if top_prec > ch_prec {
                    output.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            stack.push(ch);
        } else {
            panic!("Invalid value: {}", ch);
        }
    }

    while let Some(op) = stack.pop() {
        output.push(op);
    }

    return output.trim().to_string();
}

fn calc_postfix(expr: &str) -> i64 {
    let mut stack: Vec<i64> = Vec::new();

    for ch in expr.chars() {
        if ch.is_whitespace() {
            continue;
        } else if ch.is_digit(10) {
            let num = ch.to_digit(10).unwrap();
            stack.push(num as i64);
        } else if "-+/*".contains(ch) {
            let num2: i64 = stack.pop().unwrap();
            let num1: i64 = stack.pop().unwrap();

            let mut result: i64 = 0;

            if ch == '+' {
                result = calc_plus(num1, num2);
            }
            if ch == '-' {
                result = calc_minus(num1, num2);
            }
            if ch == '*' {
                result = calc_multiply(num1, num2);
            }
            if ch == '/' {
                result = calc_division(num1, num2);
            }

            stack.push(result)
        }
    }

    return stack.pop().unwrap();
}

fn calc_plus(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let carry = a & b;
        a = a ^ b;
        b = carry << 1;
    }

    return a;
}

fn calc_minus(a: i64, b: i64) -> i64 {
    let neg_b = calc_plus(!b, 1);
    return calc_plus(a, neg_b);
}

fn calc_multiply(a: i64, mut b: i64) -> i64 {
    let mut result = 0;
    let mut shift = 0;

    while b != 0 {
        if (b & 1) == 1 {
            result = calc_plus(result, a << shift);
        }
        shift += 1;
        b >>= 1;
    }

    return result;
}

fn calc_division(mut a: i64, mut b: i64) -> i64 {
    if b == 0 {
        panic!("Division by zero");
    }

    let mut result = 0;
    let mut sign = 1;

    if a < 0 {
        a = calc_minus(0, a);
        sign = calc_minus(0, sign);
    }
    if b < 0 {
        b = calc_minus(0, b);
        sign = calc_minus(0, sign);
    }

    let mut shift = 63;

    while shift >= 0 {
        if (a >> shift) >= b {
            a = calc_minus(a, b << shift);
            result |= 1 << shift;
        }
        if shift == 0 {
            break;
        }
        shift -= 1;
    }

    if sign < 0 {
        result = calc_minus(0, result);
    }

    return result;
}
