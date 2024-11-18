use logos::Logos;
mod lexer;
mod stack;

fn main() {
    let code = "PUSH_INT 6 PUSH_INT 5 ADD";

    let mut lex = lexer::Token::lexer(code);

    while let Some(token) = lex.next() {
        match token {
            Ok(_tk) => (), // println!("{:#?}", tk),
            Err(err) => panic!("an error occurred: {:?}", err),
        }
    }

    println!("{}", code);
    println!("extras: {:?}", lex.extras.stack)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;
    use crate::stack::{StackEl, StackElValue};
    use lexer::LexingError;
    use stack::LifoVector;

    #[test]
    fn push() {
        let code = "PUSH 69";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Int, StackElValue::Int(69))];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_push() {
        let code = "PUSH test";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => panic!("invalid push shouldn't work"),
                Err(err) => assert_eq!(err, LexingError::InvalidPush(String::from("test"))),
            }
        }
    }

    #[test]
    fn push_int() {
        let code = "PUSH_INT 6";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Int, StackElValue::Int(6))];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_push_int() {
        let code = "PUSH_INT true";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => panic!("invalid push int shouldn't work"),
                Err(err) => assert_eq!(err, LexingError::InvalidInteger(String::from("true"))),
            }
        }
    }

    #[test]
    fn push_bool() {
        let code = "PUSH_BOOL false";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Bool, StackElValue::Bool(false))];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn push_string() {
        let code = "PUSH_STR \"test\"";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(
            Token::String,
            StackElValue::String(String::from("test")),
        )];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_push_bool() {
        let code = "PUSH_BOOL 69";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => panic!("invalid push bool shouldn't work"),
                Err(err) => assert_eq!(err, LexingError::InvalidBool(String::from("69"))),
            }
        }
    }

    #[test]
    fn dup() {
        let code = "PUSH 69 DUP";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![
            StackEl::new(Token::Int, StackElValue::Int(69)),
            StackEl::new(Token::Int, StackElValue::Int(69)),
        ];
        assert_eq!(lex.extras.stack.len(), 2);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_dup() {
        let code = "DUP";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => panic!("invalid dup shouldn't work"),
                Err(err) => assert_eq!(
                    err,
                    LexingError::DupError(String::from("Stack must be at least 1 element deep"))
                ),
            }
        }
    }

    #[test]
    fn add() {
        let code = "PUSH 69 PUSH 21 ADD";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Int, StackElValue::Int(90))];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_add() {
        let code = "PUSH 69 PUSH false ADD";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // panic!("invalid add shouldn't work, stack: {:?}", lex.extras.stack),
                Err(err) => assert_eq!(
                    err,
                    LexingError::AddError(String::from("Only integers can be added"))
                ),
            }
        }
    }

    #[test]
    fn sub() {
        let code = "PUSH 21 PUSH 69 SUB";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Int, StackElValue::Int(48))];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_sub_1() {
        let code = "PUSH 20 PUSH 25 SUB";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => assert_eq!(
                    err,
                    LexingError::AddError(String::from("Subtraction overflow"))
                ),
            }
        }
    }

    #[test]
    fn invalid_sub_2() {
        let code = "PUSH 20 PUSH true SUB";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => assert_eq!(
                    err,
                    LexingError::AddError(String::from("Only integers can be subtracted"))
                ),
            }
        }
    }

    #[test]
    fn mul() {
        let code = "PUSH 20 PUSH 5 MUL";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Int, StackElValue::Int(100))];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_mul() {
        let code = "PUSH 20 PUSH true MUL";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => assert_eq!(
                    err,
                    LexingError::AddError(String::from("Only integers can be multiplied"))
                ),
            }
        }
    }

    #[test]
    fn add_mul_sub() {
        let code = "PUSH 20 PUSH 5 ADD PUSH 4 MUL PUSH 150 SUB";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Int, StackElValue::Int(50))];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn eq() {
        let code = "PUSH 20 PUSH 5 EQ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Bool, StackElValue::Bool(false))];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_eq_1() {
        let code = "PUSH 20 PUSH true EQ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => assert_eq!(
                    err,
                    LexingError::EqError(String::from("Elements must be of the same type"))
                ),
            }
        }
    }

    #[test]
    fn invalid_eq_2() {
        let code = "PUSH 20 EQ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => assert_eq!(
                    err,
                    LexingError::EqError(String::from("Stack must be at least 2 elements deep"))
                ),
            }
        }
    }

    #[test]
    fn neq() {
        let code = "PUSH 20 PUSH 5 NEQ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Bool, StackElValue::Bool(true))];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_neq_1() {
        let code = "PUSH 20 PUSH true NEQ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => assert_eq!(
                    err,
                    LexingError::NeqError(String::from("Elements must be of the same type"))
                ),
            }
        }
    }

    #[test]
    fn invalid_neq_2() {
        let code = "PUSH 20 NEQ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => assert_eq!(
                    err,
                    LexingError::NeqError(String::from("Stack must be at least 2 elements deep"))
                ),
            }
        }
    }

    #[test]
    fn pop() {
        let code = "PUSH 20 PUSH 5 POP";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Int, StackElValue::Int(20))];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_pop() {
        let code = "POP";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => assert_eq!(
                    err,
                    LexingError::PopError(String::from("Stack must be at least 1 element deep"))
                ),
            }
        }
    }

    #[test]
    fn swap() {
        let code = "PUSH 20 PUSH 5 SWAP";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![
            StackEl::new(Token::Int, StackElValue::Int(20)),
            StackEl::new(Token::Int, StackElValue::Int(5)),
        ];
        assert_eq!(lex.extras.stack.len(), 2);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_swap() {
        let code = "PUSH 69 SWAP";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => assert_eq!(
                    err,
                    LexingError::SwapError(String::from("Stack must be at least 2 elements deep"))
                ),
            }
        }
    }

    #[test]
    fn concat() {
        let code = "
            PUSH \"world\" 
            PUSH \"hello \" 
            CONCAT
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(_) => panic!("some error occurred"),
            }
        }

        let expected_stack = vec![StackEl::new(
            Token::String,
            StackElValue::String(String::from("hello world")),
        )];
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_concat() {
        let code = "
            PUSH \"world\" 
            PUSH 69 
            CONCAT
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => assert_eq!(
                    err,
                    LexingError::ConcatError(String::from("Only strings can be concatenated"))
                ),
            }
        }
    }

    #[test]
    fn jump() {
        let code = "
            PUSH \"hello world\"  
            JUMP test
            PUSH 69
            test:
                PUSH 420
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(err) => panic!("an error occurred: {:?}", err),
            }
        }

        let expected_stack = vec![
            StackEl::new(Token::Int, StackElValue::Int(420)),
            StackEl::new(
                Token::String,
                StackElValue::String(String::from("hello world")),
            ),
        ];
        // println!("stack: {:?}", lex.extras.stack);
        assert_eq!(lex.extras.stack.len(), 2);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_jump_1() {
        let code = "
            PUSH \"hello world\"  
            JUMP test2
            PUSH 69
            test2:
                PUSH 420
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => {
                    assert_eq!(err, LexingError::InvalidLabel(String::from("test2")));
                    break;
                }
            }
        }
    }

    #[test]
    fn invalid_jump_2() {
        let code = "
            PUSH \"hello world\"  
            JUMP test
            PUSH 69
            test2:
                PUSH 420
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => {
                    assert_eq!(err, LexingError::InvalidToken(String::from("test2:")));
                    break;
                }
            }
        }
    }

    #[test]
    fn invalid_jump_3() {
        let code = "
            PUSH \"hello world\"  
            PUSH 69
            test:
                PUSH 420
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => {
                    assert_eq!(err, LexingError::UnsetLabel(String::from("test:")));
                    break;
                }
            }
        }
    }

    #[test]
    fn jumpi() {
        let code = "
            PUSH \"hello world\"  
            PUSH true
            JUMPI test
            PUSH 69
            test:
                PUSH 420
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(err) => panic!("an error occurred: {:?}", err),
            }
        }

        let expected_stack = vec![
            StackEl::new(Token::Int, StackElValue::Int(420)),
            StackEl::new(
                Token::String,
                StackElValue::String(String::from("hello world")),
            ),
        ];
        // println!("stack: {:?}", lex.extras.stack);
        assert_eq!(lex.extras.stack.len(), 2);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn vector() {
        let code = "
            EMPTY_VECTOR
            PUSH 69
            INSERT
            PUSH 420
            INSERT
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(err) => panic!("an error occurred: {:?}", err),
            }
        }

        let lifo_vector = LifoVector::VectorOfInt(vec![69, 420]);
        let expected_stack = vec![StackEl::new(
            Token::InsertVector,
            StackElValue::Vector(lifo_vector),
        )];
        // println!("stack: {:?}", lex.extras.stack);
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_insert_1() {
        let code = "
            EMPTY_VECTOR
            PUSH 69
            INSERT
            PUSH true
            INSERT
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => {
                    assert_eq!(
                        err,
                        LexingError::InsertError(String::from(
                            "Cannot insert value of type bool into vector of type vector<int>"
                        ))
                    );
                    break;
                }
            }
        }
    }

    #[test]
    fn invalid_insert_2() {
        let code = "
            EMPTY_VECTOR
            INSERT
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => {
                    assert_eq!(
                        err,
                        LexingError::InsertError(String::from(
                            "Stack must be at least 2 elements deep"
                        ))
                    );
                    break;
                }
            }
        }
    }

    #[test]
    fn invalid_insert_3() {
        let code = "
            PUSH 69
            PUSH 420
            INSERT
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => {
                    assert_eq!(
                        err,
                        LexingError::InsertError(String::from(
                            "Invalid stack to insert an element in a vector"
                        ))
                    );
                    break;
                }
            }
        }
    }

    #[test]
    fn vector_size() {
        let code = "
            EMPTY_VECTOR
            PUSH 69
            INSERT
            PUSH 420
            INSERT
            SIZE
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(err) => panic!("an error occurred: {:?}", err),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Size, StackElValue::Int(2))];
        // println!("stack: {:?}", lex.extras.stack);
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn string_size() {
        let code = "
            PUSH \"hello world\"
            SIZE
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(err) => panic!("an error occurred: {:?}", err),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Size, StackElValue::Int(11))];
        // println!("stack: {:?}", lex.extras.stack);
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn invalid_size() {
        let code = "
            PUSH false
            SIZE
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (),
                Err(err) => {
                    assert_eq!(
                        err,
                        LexingError::SizeError(String::from(
                            "Cannot give the size of element of type bool"
                        ))
                    );
                    break;
                }
            }
        }
    }

    #[test]
    fn vector_index() {
        let code = "
            EMPTY_VECTOR
            PUSH 69
            INSERT
            PUSH 420
            INSERT
            INDEX 1
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(err) => panic!("an error occurred: {:?}", err),
            }
        }

        let expected_stack = vec![StackEl::new(Token::Index, StackElValue::Int(420))];
        // println!("stack: {:?}", lex.extras.stack);
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn string_index() {
        let code = "
            PUSH \"hello world\"
            INDEX 6
        ";
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(err) => panic!("an error occurred: {:?}", err),
            }
        }

        let expected_stack = vec![StackEl::new(
            Token::Index,
            StackElValue::String(String::from("w")),
        )];
        // println!("stack: {:?}", lex.extras.stack);
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn comments() {
        let code = r#"
            PUSH 69             /* this is a comment */
            PUSH "hello world"
        "#;
        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(err) => panic!("an error occurred: {:?}", err),
            }
        }

        let expected_stack = vec![
            StackEl::new(
                Token::String,
                StackElValue::String(String::from("hello world")),
            ),
            StackEl::new(Token::Int, StackElValue::Int(69)),
        ];
        // println!("stack: {:?}", lex.extras.stack);
        assert_eq!(lex.extras.stack.len(), 2);
        assert_eq!(lex.extras.stack, expected_stack);
    }

    #[test]
    fn final_test() {
        let code = r#"
            EMPTY_VECTOR                    /* [] */
            PUSH "hello "                   /* "hello ", [] */
            INSERT                          /* ["hello "] */
            PUSH "world"                    /* "world", ["hello "] */
            INSERT                          /* ["hello ", "world"] */
            DUP                             /* ["hello ", "world"]], ["hello ", "world"] */
            INDEX 1                         /* "world", ["hello ", "world"] */
            SWAP                            /* ["hello ", "world"], "world" */
            INDEX 0                         /* "hello ", "world */
            JUMP test                       /* "hello ", "world */
            PUSH 69                         /* "hello ", "world */
            PUSH_INT 420                    /* "hello ", "world */
            test:                           /* "hello ", "world */
                CONCAT                      /* "hello world" */
                DUP                         /* "hello world", "hello world" */
                SIZE                        /* 11, "hello world" */
                PUSH 11                     /* 11, 11, "hello world" */
                EQ                          /* true, "hello world" */
                POP                         /* "hello world" */
                PUSH "hello world"          /* "hello world", "hello world" */
                EQ                          /* true */
                PUSH_BOOL false             /* false, true */
                NEQ                         /* true */
        "#;

        let mut lex = lexer::Token::lexer(code);
        while let Some(token) = lex.next() {
            match token {
                Ok(_tk) => (), // println!("{:#?}", tk),
                Err(err) => {
                    println!("extras: {:?}", lex.extras);
                    panic!("an error occurred: {:?}", err)
                }
            }
        }

        let expected_stack = vec![StackEl::new(Token::Bool, StackElValue::Bool(true))];
        // println!("stack: {:?}", lex.extras.stack);
        assert_eq!(lex.extras.stack.len(), 1);
        assert_eq!(lex.extras.stack, expected_stack);
    }
}
