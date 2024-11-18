use crate::stack::{add, concat, dup, eq, mul, neq, pop, sub, swap, insert_vector, size, index, Stack, StackEl, StackElValue, LifoVector};
use logos::{Lexer, Logos};

fn label_is_set(current_label: &Option<String>) -> bool {
    // println!("current label: {:?}", current_label);
    match current_label {
        None => false,
        Some(_) => true,
    }
}

#[derive(Default, Debug)]
pub struct State {
    pub stack: Stack,
    pub prev_op: Token,
    current_label: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidInteger(String),
    InvalidBool(String),
    InvalidString(String),
    InvalidPush(String),
    InvalidLabel(String),
    InvalidOpcode(String),
    InvalidToken(String),
    UnsetLabel(String),
    AddError(String),
    DupError(String),
    EqError(String),
    NeqError(String),
    PopError(String),
    SwapError(String),
    ConcatError(String),
    JumpiError(String),
    InsertError(String),
    SizeError(String),
    IndexError(String),
    #[default]
    Unknown,
}

#[derive(Logos, Debug, PartialEq, Clone, Default)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(extras = State)]
#[logos(error = LexingError)]
pub enum Token {
    #[token("ADD", op_add)]
    Add,

    #[token("SUB", op_sub)]
    Sub,

    #[token("MUL", op_mul)]
    Mul,

    #[regex("PUSH", op_push)]
    Push,

    #[regex("PUSH_INT", op_push_int)]
    PushInt,

    #[regex("PUSH_BOOL", op_push_bool)]
    PushBool,

    #[regex("PUSH_STR", op_push_str)]
    PushStr,

    #[regex("[0-9]+", priority = 1)]
    Int,

    #[regex("true|false")]
    Bool,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#)]
    String,

    #[regex("DUP", op_dup)]
    Dup,

    #[regex("EQ", op_eq)]
    Eq,

    #[regex("NEQ", op_neq)]
    Neq,

    #[regex("POP", op_pop)]
    Pop,

    #[regex("SWAP", op_swap)]
    Swap,

    #[regex("CONCAT", op_concat)]
    Concat,

    #[regex("JUMP", op_jump)]
    Jump,

    #[regex("JUMPI", op_jumpi)]
    Jumpi,

    #[regex("EMPTY_VECTOR", op_empty_vector)]
    EmptyVector,

    #[regex("INSERT", op_insert_vector)]
    InsertVector,

    #[regex("SIZE", op_size)]
    Size,

    #[regex("INDEX", op_index)]
    Index,

    #[regex("LOG", op_log)]
    Log,

    #[regex("[a-z_]+", check_label_name)]
    LabelName,

    #[regex("[a-z_]+:", |lex| {
        match &lex.extras.current_label {
            None => Err(LexingError::UnsetLabel(lex.slice().to_string())),
            Some(current_label) =>
                {
                    let mut label = lex.slice().to_string();
                    let _ = label.pop(); // removes the colon at the end of the string
                    if &label == current_label {
                        // same label
                        lex.extras.current_label = None;
                    }
                    Ok(())
                }
        }
    })]
    Label,

    // #[regex(r"/\*([^*]|\**[^*/])*\*+/")]
    // Comment,

    #[token("/*", |lex| {
        let len = lex.remainder().find("*/")?;
        lex.bump(len + 2); // include len of `*/`
    
        Some(())
    })]
    Comment,

    #[default]
    #[regex(
        "[a-zA-Z0-9_:-]+", 
        priority = 0, 
        callback = invalid_token
    )]
    Invalid,
}

fn invalid_token(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if lex.extras.prev_op == Token::Jump {
        Err(LexingError::InvalidLabel(lex.slice().to_string()))
    } else {
        Err(LexingError::InvalidToken(lex.slice().to_string()))
    }
}

fn op_push(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }

    match lex.next() {
        Some(Ok(Token::Int)) => {
            let val = lex.slice().parse().unwrap();
            lex.extras
                .stack
                .insert(0, StackEl::new(Token::Int, StackElValue::Int(val)));
            Ok(())
        }
        Some(Ok(Token::Bool)) => {
            let val = lex.slice().parse().unwrap();
            lex.extras
                .stack
                .insert(0, StackEl::new(Token::Bool, StackElValue::Bool(val)));
            Ok(())
        }
        Some(Ok(Token::String)) => {
            let str_value = &lex.slice()[1..lex.slice().len() - 1];
            lex.extras
                .stack
                .insert(0, StackEl::new(Token::String, StackElValue::String(str_value.to_string())));
            Ok(())
        }
        _ => Err(LexingError::InvalidPush(String::from(lex.slice()))),
    }
}

fn op_push_int(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }

    match lex.next() {
        Some(Ok(Token::Int)) => {
            let val = lex.slice().parse().unwrap();
            lex.extras
                .stack
                .push(StackEl::new(Token::Int, StackElValue::Int(val)));
            Ok(())
        }
        _ => Err(LexingError::InvalidInteger(String::from(lex.slice()))),
    }
}

fn op_push_str(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match lex.next() {
        Some(Ok(Token::String)) => {
            let str_value = &lex.slice()[1..lex.slice().len() - 1];
            lex.extras
                .stack
                .push(StackEl::new(Token::String, StackElValue::String(str_value.to_string())));
            Ok(())
        }
        _ => Err(LexingError::InvalidString(String::from(lex.slice()))),
    }
}

fn op_push_bool(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match lex.next() {
        Some(Ok(Token::Bool)) => {
            let val = lex.slice().parse().unwrap();
            lex.extras
                .stack
                .push(StackEl::new(Token::Bool, StackElValue::Bool(val)));
            Ok(())
        }
        _ => Err(LexingError::InvalidBool(String::from(lex.slice()))),
    }
}

fn op_add(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match add(&lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
            Ok(())
        }
        Err(err) => Err(LexingError::AddError(err)),
    }
}

fn op_sub(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match sub(&lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
            Ok(())
        }
        Err(err) => Err(LexingError::AddError(err)),
    }
}

fn op_mul(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match mul(&lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
            Ok(())
        }
        Err(err) => Err(LexingError::AddError(err)),
    }
}

fn op_dup(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match dup(&lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
            Ok(())
        }
        Err(err) => Err(LexingError::DupError(err)),
    }
}

fn op_eq(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match eq(&lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
            Ok(())
        }
        Err(err) => Err(LexingError::EqError(err)),
    }
}

fn op_neq(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match neq(&lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
            Ok(())
        }
        Err(err) => Err(LexingError::NeqError(err)),
    }
}

fn op_pop(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match pop(&lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
            Ok(())
        }
        Err(err) => Err(LexingError::PopError(err)),
    }
}

fn op_swap(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match swap(&mut lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
            Ok(())
        }
        Err(err) => Err(LexingError::SwapError(err)),
    }
}

fn op_concat(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match concat(&mut lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
            Ok(())
        }
        Err(err) => Err(LexingError::ConcatError(err)),
    }
}

fn op_jump(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    lex.extras.prev_op = Token::Jump;

    match lex.next() {
        Some(Ok(Token::LabelName)) => {
            let label = lex.slice();
            lex.extras.current_label = Some(label.to_string());
            Ok(())
        }
        _ => Err(LexingError::InvalidLabel(String::from(lex.slice()))),
    }
}

fn check_label_name(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    match lex.extras.prev_op {
        Token::Jump | Token::Jumpi => {
            lex.extras.prev_op = Token::Invalid;
            Ok(())
        }
        _ => Err(LexingError::InvalidOpcode(String::from(lex.slice()))),
    }
}

fn op_jumpi(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if lex.extras.stack.len() < 1 {
        return Err(LexingError::JumpiError(String::from("Stack must be at least 1 element deep")));
    }

    if lex.extras.stack[0].token != Token::Bool {
        return Err(LexingError::JumpiError(String::from("Top element must be a boolean value")));
    }

    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    lex.extras.prev_op = Token::Jumpi;

    match lex.next() {
        Some(Ok(Token::LabelName)) => {
            if lex.extras.stack[0].value == StackElValue::Bool(true) {
                let label = lex.slice();
                lex.extras.current_label = Some(label.to_string());
            }
            // removes the boolean value on the stack
            lex.extras.stack = lex.extras.stack[1..].to_vec();

            Ok(())
        }
        _ => Err(LexingError::InvalidLabel(String::from(lex.slice()))),
    }
}

fn op_empty_vector(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    let new_value = 
        StackEl::new(
            Token::EmptyVector, 
            StackElValue::Vector(LifoVector::new())
        );
    lex.extras.stack = vec![vec![new_value], lex.extras.stack.clone()].concat();

    Ok(())
}

fn op_insert_vector(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }

    match insert_vector(&mut lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
        
            Ok(())
        }
        Err(err) => Err(LexingError::InsertError(err))
    }

}

fn op_size(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }
    
    match size(&mut lex.extras.stack) {
        Ok(new_stack) => {
            lex.extras.stack = new_stack;
            Ok(())
        }
        Err(err) => Err(LexingError::SizeError(err)),
    }
}

fn op_index(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    if label_is_set(&lex.extras.current_label) {
        return Ok(());
    }

    match lex.next() {
        Some(Ok(Token::Int)) => {
            let val = lex.slice().parse().unwrap();
            match index(&mut lex.extras.stack, val) {
                Ok(new_stack) => {
                    lex.extras.stack = new_stack;
                    Ok(())
                },
                Err(err) => Err(LexingError::IndexError(err))
            }
        }
        _ => Err(LexingError::InvalidInteger(String::from(lex.slice()))),
    }
}

fn op_log(lex: &mut Lexer<Token>) -> Result<(), LexingError> {
    println!("current stack: {:?}", lex.extras.stack);

    Ok(())
}