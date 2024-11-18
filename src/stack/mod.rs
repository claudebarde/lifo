use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum LifoVector {
    EmptyVector,
    VectorOfInt(Vec<usize>),
    VectorOfBool(Vec<bool>),
    VectorOfString(Vec<String>),
}
impl LifoVector {
    pub fn new() -> Self {
        return LifoVector::EmptyVector;
    }

    pub fn insert(self, el: StackElValue) -> Result<Self, String> {
        match (self.clone(), el.clone()) {
            // vector is empty
            (LifoVector::EmptyVector, StackElValue::Int(val)) => {
                Ok(LifoVector::VectorOfInt(vec![val]))
            }
            (LifoVector::EmptyVector, StackElValue::Bool(val)) => {
                Ok(LifoVector::VectorOfBool(vec![val]))
            }
            (LifoVector::EmptyVector, StackElValue::String(val)) => {
                Ok(LifoVector::VectorOfString(vec![val]))
            }
            // vector is already populated
            (LifoVector::VectorOfBool(mut vector), StackElValue::Bool(val)) => {
                vector.push(val);
                Ok(LifoVector::VectorOfBool(vector))
            }
            (LifoVector::VectorOfInt(mut vector), StackElValue::Int(val)) => {
                vector.push(val);
                Ok(LifoVector::VectorOfInt(vector))
            }
            (LifoVector::VectorOfString(mut vector), StackElValue::String(val)) => {
                vector.push(val);
                Ok(LifoVector::VectorOfString(vector))
            }
            _ => {
                let vector_type = match self {
                    LifoVector::EmptyVector => "unknown",
                    LifoVector::VectorOfBool(_) => "bool",
                    LifoVector::VectorOfInt(_) => "int",
                    LifoVector::VectorOfString(_) => "string",
                };

                Err(format!(
                    "Cannot insert value of type {} into vector of type vector<{}>",
                    el.to_string(),
                    vector_type
                ))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackElValue {
    Int(usize),
    String(String),
    Bool(bool),
    Vector(LifoVector),
}
impl StackElValue {
    pub fn to_string(self) -> String {
        match self {
            StackElValue::Bool(_) => String::from("bool"),
            StackElValue::Int(_) => String::from("int"),
            StackElValue::String(_) => String::from("string"),
            StackElValue::Vector(_) => String::from("vector"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackEl {
    pub token: Token,
    pub value: StackElValue,
}
impl StackEl {
    pub fn new(tk: Token, val: StackElValue) -> Self {
        StackEl {
            token: tk,
            value: val,
        }
    }

    pub fn print(self) -> String {
        match &self.value {
            StackElValue::Bool(val) => format!("{} : bool", val),
            StackElValue::Int(val) => format!("{} : int", val),
            StackElValue::String(val) => format!("{} : string", val),
            StackElValue::Vector(val) => format!("{:?} : vector", val),
        }
    }
}

pub type Stack = Vec<StackEl>;

pub fn add(stack: &Stack) -> Result<Stack, String> {
    if stack.len() < 2 {
        return Err(String::from("Stack must be at least 2 elements deep"));
    }

    match (
        &stack[0].token,
        &stack[0].value,
        &stack[1].token,
        &stack[1].value,
    ) {
        (Token::Int, StackElValue::Int(val1), Token::Int, StackElValue::Int(val2)) => {
            let new_value = val1 + val2;
            let new_stack = vec![
                vec![StackEl::new(Token::Int, StackElValue::Int(new_value))],
                stack[2..].to_vec(),
            ]
            .concat();

            Ok(new_stack)
        }
        _ => Err(String::from("Only integers can be added")),
    }
}

pub fn sub(stack: &Stack) -> Result<Stack, String> {
    if stack.len() < 2 {
        return Err(String::from("Stack must be at least 2 elements deep"));
    }

    match (
        &stack[0].token,
        &stack[0].value,
        &stack[1].token,
        &stack[1].value,
    ) {
        (Token::Int, StackElValue::Int(minuend), Token::Int, StackElValue::Int(subtrahend)) => {
            if minuend < subtrahend {
                return Err(String::from("Subtraction overflow"));
            }
            let new_value = minuend - subtrahend;

            let new_stack = vec![
                vec![StackEl::new(Token::Int, StackElValue::Int(new_value))],
                stack[2..].to_vec(),
            ]
            .concat();

            Ok(new_stack)
        }
        _ => Err(String::from("Only integers can be subtracted")),
    }
}

pub fn mul(stack: &Stack) -> Result<Stack, String> {
    if stack.len() < 2 {
        return Err(String::from("Stack must be at least 2 elements deep"));
    }

    match (
        &stack[0].token,
        &stack[0].value,
        &stack[1].token,
        &stack[1].value,
    ) {
        (
            Token::Int,
            StackElValue::Int(multiplicand),
            Token::Int,
            StackElValue::Int(multiplier),
        ) => {
            let new_value = multiplicand * multiplier;
            let new_stack = vec![
                vec![StackEl::new(Token::Int, StackElValue::Int(new_value))],
                stack[2..].to_vec(),
            ]
            .concat();

            Ok(new_stack)
        }
        _ => Err(String::from("Only integers can be multiplied")),
    }
}

pub fn dup(stack: &Stack) -> Result<Stack, String> {
    if stack.len() < 1 {
        return Err(String::from("Stack must be at least 1 element deep"));
    }

    let new_value = stack[0].clone();
    Ok(vec![vec![new_value], stack.clone()].concat())
}

pub fn eq(stack: &Stack) -> Result<Stack, String> {
    if stack.len() < 2 {
        return Err(String::from("Stack must be at least 2 elements deep"));
    }

    if stack[0].value.clone().to_string() != stack[1].value.clone().to_string() {
        return Err(String::from("Elements must be of the same type"));
    }

    let new_value = if stack[0].value == stack[1].value {
        StackEl::new(Token::Bool, StackElValue::Bool(true))
    } else {
        StackEl::new(Token::Bool, StackElValue::Bool(false))
    };

    let new_stack = vec![vec![new_value], stack[2..].to_vec()].concat();

    Ok(new_stack)
}

pub fn neq(stack: &Stack) -> Result<Stack, String> {
    if stack.len() < 2 {
        return Err(String::from("Stack must be at least 2 elements deep"));
    }

    if stack[0].token != stack[1].token {
        return Err(String::from("Elements must be of the same type"));
    }

    let new_value = if stack[0].value == stack[1].value {
        StackEl::new(Token::Bool, StackElValue::Bool(false))
    } else {
        StackEl::new(Token::Bool, StackElValue::Bool(true))
    };

    let new_stack = vec![vec![new_value], stack[2..].to_vec()].concat();

    Ok(new_stack)
}

pub fn pop(stack: &Stack) -> Result<Stack, String> {
    if stack.len() < 1 {
        return Err(String::from("Stack must be at least 1 element deep"));
    }

    Ok(stack[1..].to_vec())
}

pub fn swap(stack: &mut Stack) -> Result<Stack, String> {
    if stack.len() < 2 {
        return Err(String::from("Stack must be at least 2 elements deep"));
    }

    stack.swap(0, 1);

    Ok(stack.to_vec())
}

pub fn concat(stack: &mut Stack) -> Result<Stack, String> {
    if stack.len() < 2 {
        return Err(String::from("Stack must be at least 2 elements deep"));
    }

    match (
        &stack[0].token,
        &stack[0].value,
        &stack[1].token,
        &stack[1].value,
    ) {
        (
            Token::String | Token::Index,
            StackElValue::String(val1),
            Token::String | Token::Index,
            StackElValue::String(val2),
        ) => {
            let new_value = format!("{}{}", val1, val2);
            let mut new_stack = stack[2..].to_vec();
            new_stack.insert(
                0,
                StackEl::new(Token::String, StackElValue::String(new_value)),
            );

            Ok(new_stack)
        }
        _ => Err(String::from("Only strings can be concatenated")),
    }
}

pub fn insert_vector(stack: &mut Stack) -> Result<Stack, String> {
    if stack.len() < 2 {
        return Err(String::from("Stack must be at least 2 elements deep"));
    }

    match (&stack[0].value, &stack[1].value) {
        (val_to_insert, StackElValue::Vector(lifo_vector)) => {
            let new_vector = lifo_vector.clone().insert(val_to_insert.clone())?;
            let stack_val = StackElValue::Vector(new_vector);
            // removes the 2 values on top of the stack and pushes the new vector
            let new_stack = vec![
                vec![StackEl::new(Token::InsertVector, stack_val)],
                stack[2..].to_vec(),
            ]
            .concat();

            Ok(new_stack)
        }
        _ => Err(String::from(
            "Invalid stack to insert an element in a vector",
        )),
    }
}

pub fn size(stack: &Stack) -> Result<Stack, String> {
    if stack.len() < 1 {
        return Err(String::from("Stack must be at least 1 element deep"));
    }

    let new_value = match stack[0].clone().value {
        StackElValue::String(val) => Ok(StackEl::new(Token::Size, StackElValue::Int(val.len()))),
        StackElValue::Vector(val) => {
            let size = match val {
                LifoVector::EmptyVector => 0,
                LifoVector::VectorOfBool(vec) => vec.len(),
                LifoVector::VectorOfInt(vec) => vec.len(),
                LifoVector::VectorOfString(vec) => vec.len(),
            };
            Ok(StackEl::new(Token::Size, StackElValue::Int(size)))
        }
        _ => Err(format!(
            "Cannot give the size of element of type {}",
            stack[0].value.clone().to_string()
        )),
    }?;

    let new_stack = vec![vec![new_value], stack[1..].to_vec()].concat();

    Ok(new_stack)
}

pub fn index(stack: &Stack, index: usize) -> Result<Stack, String> {
    if stack.len() < 1 {
        return Err(String::from("Stack must be at least 1 element deep"));
    }

    let new_value = match stack[0].clone().value {
        StackElValue::String(val) => {
            if val.len() < index {
                Err(format!(
                    "Out of bound index {} for string of length {}",
                    index,
                    val.len()
                ))
            } else {
                Ok(StackEl::new(
                    Token::Index,
                    StackElValue::String(val.chars().nth(index).unwrap().to_string()),
                ))
            }
        }
        StackElValue::Vector(val) => match val {
            LifoVector::EmptyVector => {
                Err(format!("Out of bound index {} for empty vector", index))
            }
            LifoVector::VectorOfBool(vec) => {
                if vec.len() < index {
                    Err(format!(
                        "Out of bound index {} for vector of length {}",
                        index,
                        vec.len()
                    ))
                } else {
                    Ok(StackEl::new(Token::Index, StackElValue::Bool(vec[index])))
                }
            }
            LifoVector::VectorOfInt(vec) => {
                if vec.len() < index {
                    Err(format!(
                        "Out of bound index {} for vector of length {}",
                        index,
                        vec.len()
                    ))
                } else {
                    Ok(StackEl::new(Token::Index, StackElValue::Int(vec[index])))
                }
            }
            LifoVector::VectorOfString(vec) => {
                if vec.len() < index {
                    Err(format!(
                        "Out of bound index {} for vector of length {}",
                        index,
                        vec.len()
                    ))
                } else {
                    Ok(StackEl::new(
                        Token::Index,
                        StackElValue::String(vec[index].to_string()),
                    ))
                }
            }
        },
        _ => Err(format!(
            "Cannot index element of type {}",
            stack[0].value.clone().to_string()
        )),
    }?;

    let new_stack = vec![vec![new_value], stack[1..].to_vec()].concat();

    Ok(new_stack)
}
