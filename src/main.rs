use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::env;
use crate::TokenType::Operator;
use std::clone::Clone;
use crate::tokenize::Lexer;

mod tokenize;

// fn main() {
//     println!("Hello, world!");
// }

#[derive(PartialEq, Clone, Debug, Eq)]
pub enum TokenType {
    EOF,
    Illegal,
    Identifier,
    IntegerLiteral,
    CharLiteral,
    Operator,
    Array,
    Begin,
    Const,
    Do,
    Else,
    End,
    Func,
    If,
    In,
    Let,
    Of,
    Proc,
    Record,
    Then,
    Type,
    Var,
    While,
    Period,
    Colon,
    Semicolon,
    Comma,
    Equals,
    Tilde,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
}

// token = intlit, lexeme = "123", row = 1, col = 1

enum SyntaxError {
    UnexpectedToken {
        expected:TokenType,
        found: TokenType,
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub row: usize,
    pub col: usize,
}

pub struct Parser {
    pub current_token: Token,
    pub tokens: Vec<Token>,
    pub index: usize
}

impl Parser {
    fn parser_accept(&mut self, expected: TokenType) -> Result<(), SyntaxError> {
        if self.current_token.token_type == expected {
            self.next_token();
            Ok(())
        } else {
            Err(SyntaxError::UnexpectedToken {
                expected,
                found: self.current_token.token_type.clone(),
            })
        }
    }


    fn parser_accept_it(&mut self) {
            self.next_token();
        }

        fn next_token(&mut self) {
            if self.index < self.tokens.len() - 1 {
                self.index += 1;
                self.current_token = self.tokens[self.index].clone();
            }
        }

        fn parser_parse(&mut self) {
            self.parser_accept_it();
            self.parse_command();
        }


        // Command -> single-command Cmd’
        fn parse_command(&mut self) {
            self.parse_single_command();
            self.parse_command_prime();
        }
        // Cmd’ -> ;single-command Cmd’| e
        fn parse_command_prime(&mut self) -> Result<(), SyntaxError> {
            if self.current_token.token_type == TokenType::Semicolon {
                self.parser_accept_it();
                self.parse_single_command();
                self.parse_command_prime()
            } else {
                Ok(()) // empty sentence ε
            }
        }

        // single-Command ::= identitifer single-Command'
        //                    | begin Declaration in single-Command
        //                    | if Expression then single-Command else single-Command
        //                    | while Expression do single-Command

        fn parse_single_command(&mut self) {
            match self.current_token.token_type {
                TokenType::Identifier => {
                    self.parser_accept_it();
                    self.parse_single_command_prime();
                },
                TokenType::Begin => {
                    self.parse_declaration();
                    self.parser_accept_it();
                    self.parser_accept(TokenType::In);
                    self.parse_single_command();
                },
                TokenType::If => {
                    self.parse_expression();
                    self.parser_accept_it();
                    self.parser_accept(TokenType::Then);
                    self.parse_single_command();
                    self.parser_accept_it();
                    self.parser_accept(TokenType::Else);
                    self.parse_single_command()
                },
                TokenType::While => {
                    self.parse_expression();
                    self.parser_accept_it();
                    self.parser_accept(TokenType::Do);
                    self.parse_single_command()
                },
                // _ => Err(SyntaxError::UnexpectedToken {
                //     expected: TokenType::Const,
                //     found: self.current_token.token_type.clone(),
                // }),
                _ => todo!(), //el compilador dice
            }
        }

        // CORRECCION
        //single-Command':=
        //              | Vn' := Expression
        //              | ( Actual-Parameter-Sequence ) -> APSequence
        fn parse_single_command_prime(&mut self) {
            if self.current_token.token_type == TokenType::LeftParen {
                //self.parse_ap_sequence();
                self.parser_accept_it();
                self.parser_accept(TokenType::RightParen);
                //self.parse_apsequence()
            } else {
                self.parse_vname_prime();
            }
        }
        //Declaration -> single-Declaration D’
        //D’ -> ; single-Declaration D’|e
        fn parse_declaration(&mut self) {
            if self.current_token.token_type == TokenType::Semicolon {
                self.parser_accept_it();
                self.parse_single_declaration();
                self.parse_declaration_prime();
            } else {
                //Ok(()); // empty sentence ε
            }
        }

        fn parse_declaration_prime(&mut self) {
            if self.current_token.token_type == TokenType::Semicolon {
                self.parser_accept_it();
                self.parse_single_declaration();
                self.parse_declaration_prime();
            } else {
                //Ok(()); // empty sentence ε
            }
        }

        // ******************************************************************
        // Actual-Param-Seq ::= e                                         //*
        //                  | proper-AP-Sequence                          //*
        // fn parseAPSequence(&mut self) -> Result<(), SyntaxError> {     //*
        //     self.parser_accept(TokenType::LeftParen);                  //*
        //     self.parseActualParameterSequence();                       //*
        //     self.parser_accept_it();                                   //*
        //     self.parser_accept(Token.tokenType.RightParen);            //*
        // }                                                              //*
        //                                                                //*        NOTA: Preguntar al Compañero y al Profesor
        //                                                                //*
        // // Proper-AP-Secuence -> Actual-Parameter pAPS’                //*
        // // pAPS’-> e|                                                  //*
        // //         ,Proper-AP-Secuence                                 //*
        // fn parseProperAPSequence(){                                    //*
        //     parseActualParameter();                                    //*
        //     parseProperAPSequencePrime();                              //*
        // }                                                              //*
        //                                                                //*
        // fn parseProperAPSequencePrime(){                               //*
        //     //empty sentence                                           //*
        //     parseProperAPSequence();                                   //*
        //}                                                               //*
        // ******************************************************************

        // ----------------------------- MELI -----------------------------

        // single-Declaration ::= const Identifier ~ Expression
        // | var Identifier : Type-denoter
        // | proc Identififer ( Formal-Parameter-Sequence ) ~ single-Command
        // | func Identififer ( Formal-Parameter-Sequence ) : Type-denoter ~ Expression
        // | type Identifier ~ Type-denoter
        fn parse_single_declaration(&mut self) -> Result<(), SyntaxError> {
            self.parser_accept_it();
            match self.current_token.token_type {
                TokenType::Const => {
                    self.parser_accept(TokenType::Identifier)?;
                    self.parser_accept(TokenType::Tilde)?; // ~
                    Ok(self.parse_expression())
                },
                TokenType::Var => {
                    self.parser_accept(TokenType::Colon)?;
                    Ok(self.parse_type_denoter()?)
                },
                TokenType::Proc => {
                    self.parser_accept(TokenType::Identifier)?;
                    self.parser_accept(TokenType::LeftParen)?;
                    self.parse_formal_parameter_sequence()?;
                    self.parser_accept_it();
                    self.parser_accept(TokenType::RightParen)?;
                    self.parser_accept(TokenType::Tilde)?;
                    Ok(self.parse_single_command())
                },
                TokenType::Func => {
                    self.parser_accept(TokenType::Identifier)?;
                    self.parser_accept(TokenType::LeftParen)?;
                    self.parse_formal_parameter_sequence()?;
                    self.parser_accept_it();
                    self.parser_accept(TokenType::RightParen)?;
                    self.parser_accept(TokenType::Colon)?;
                    self.parse_type_denoter()?;
                    self.parser_accept_it();
                    self.parser_accept(TokenType::Tilde)?;
                    Ok(self.parse_expression())
                },
                TokenType::Type => {
                    self.parser_accept(TokenType::Identifier)?;
                    self.parser_accept(TokenType::Tilde)?;
                    self.parse_type_denoter()?;
                    self.parser_accept_it();
                    Ok(self.parse_expression())
                },
                _ => Err(SyntaxError::UnexpectedToken {
                    expected: TokenType::Const,
                    found: self.current_token.token_type.clone(),
                }),
            }
        }


        // Formal-Param-Seq ::= e
        //                      | proper-FP-Sequence
        fn parse_formal_parameter_sequence(&mut self) -> Result<(), SyntaxError> {
            self.parser_accept_it();
            if let TokenType::Identifier | TokenType::Var | TokenType::Proc | TokenType::Func = self.current_token.token_type {
                Ok(self.parse_proper_fpsequence())
            } else {
                Ok(()) // empty sentence ε
            }
        }


        // proper-FP-Sequence -> Formal-Parameter pFPS’
        // pFPS’ -> e|,proper-FP-Secuence
        fn parse_proper_fpsequence(&mut self) {
            {
                //self.parser_accept_it();
                self.parse_formal_parameter_sequence();
                self.parse_proper_fpsequence_prime();
            }
        }

        fn parse_proper_fpsequence_prime(&mut self) {
            self.parser_accept_it();
            if self.current_token.token_type == TokenType::Comma {
                self.parse_proper_fpsequence()
            } else {
                //Ok(());// ε  empty sentence
            }
        }

        // Formal-Parameter ::= Identifier : Type-denoter
        // | var Identifier : Type-denoter
        // | proc Identifier ( Formal-Param-Seq )
        // | func Identifier ( Formal-Param-Seq ) : Type-denoter

        fn parse_formal_parameter(mut self) ->Result<(), SyntaxError>{
            self.parser_accept_it();
            match self.current_token.token_type {
                TokenType::Identifier => {
                    self.parser_accept(TokenType::Colon)?;
                    self.parse_type_denoter()
                },
                TokenType::Var => {
                    self.parser_accept(TokenType::Identifier)?;
                    self.parser_accept(TokenType::Colon)?;
                    self.parse_type_denoter()
                },
                TokenType::Proc | TokenType::Func => {
                    self.parser_accept(TokenType::Identifier)?;
                    self.parser_accept(TokenType::LeftParen)?;
                    self.parse_formal_parameter_sequence()?;
                    self.parser_accept_it();
                    self.parser_accept(TokenType::RightParen)?;
                    self.parser_accept(TokenType::Colon)?;
                    self.parse_type_denoter()
                },
                _ => Err(SyntaxError::UnexpectedToken {
                    expected: TokenType::Identifier,
                    found: self.current_token.token_type,
                }),
            }
        }


        // Actual-Parameter ::= Expression
        // | var V-name
        // | proc Identifier
        // | func Identifier

        fn parse_actual_parameter(&mut self) {
            self.parser_accept_it();
            match self.current_token.token_type {
                TokenType::Var => {
                    self.parser_accept_it();
                    self.parse_vname()
                },
                TokenType::Proc | TokenType::Func => {
                    self.parser_accept_it();
                    self.parser_accept(TokenType::Identifier);
                },
                _ => self.parse_expression(),
            }
        }

        // Type-denoter ::= Identifier
        //                  | array Integer-Literal of Type-denoter
        //                  | record Record-Type-denoter end

        fn parse_type_denoter(&mut self) -> Result<(), SyntaxError> {
            match self.current_token.token_type {
                TokenType::Identifier => {
                    self.parser_accept_it();
                    Ok(())
                },
                TokenType::Array => {
                    self.parser_accept_it();
                    self.parser_accept(TokenType::IntegerLiteral)?;
                    self.parser_accept(TokenType::Of)?;
                    self.parse_type_denoter()
                },
                TokenType::Record => {
                    self.parser_accept_it();
                    self.parse_record_type_denoter()?;
                    self.parser_accept_it();
                    self.parser_accept(TokenType::End)
                },
                _ => Err(SyntaxError::UnexpectedToken {
                    expected: TokenType::Identifier,
                    found: self.current_token.token_type.clone(),
                }),
            }
        }


        // Record-Type-denoter -> identifier : Type-denoter RTd’
        // RTd’ -> e|,Record-Type-denoter
        fn parse_record_type_denoter(&mut self) -> Result<(), SyntaxError> {
            self.parser_accept(TokenType::Identifier)?;
            self.parser_accept(TokenType::Colon)?;
            self.parse_type_denoter()?;
            self.parser_accept_it();
            self.parse_record_type_denoter_prime()
        }

        fn parse_record_type_denoter_prime(&mut self) -> Result<(), SyntaxError> {
            if self.current_token.token_type == TokenType::Comma {
                self.parse_record_type_denoter()
            } else {
                Ok(())// ε empty sentence
            }
        }


        //++++++++++++++++++++++++++++++++++++++++++++Matias++++++++++++++++++++++++++++++++++++++++++++

        // Expression ::= second-Expression
        // | let Declaration in Expression
        // | if Expression then Expression else Expression
        fn parse_expression(&mut self) {
            if self.current_token.token_type == TokenType::Let {
                self.parser_accept_it();
                self.parse_declaration();
                self.parser_accept(TokenType::In);
                self.parse_expression();
            } else if self.current_token.token_type == TokenType::If {
                self.parser_accept_it();
                self.parse_expression();
                self.parser_accept(TokenType::Then);
                self.parse_expression();
                self.parser_accept(TokenType::Else);
                self.parse_expression();
            } else {
                self.parse_second_expression();
            }
        }

        // second-Expression -> primary-Expresion sE’
        // sE’-> Operator primary -Expresion sE’| e
        fn parse_second_expression(&mut self) {
            self.parse_primary_expression();
            self.parse_seprime();
        }

        fn parse_seprime(&mut self) {
            if self.current_token.token_type == TokenType::Operator {
                self.parser_accept_it();
                self.parse_primary_expression();
                self.parse_seprime();
            } else {
                //Ok(());
            }
        }

        //PREGUNTAR
        //********************************************************************************
        // primary-Expression ::= Integer-Literal                                      //*
        // | Character-Literal                                                         //*
        // | identitifer (Vn’ ( Actual-Parameter-Sequence ))* ?????????                //*
        // | Operator primary-Expression                                               //*
        // | ( Expression )                                                            //*
        // | { Record-Aggregate }                                                      //*
        // | [ Array-Aggregate ]                                                       //*
        fn parse_primary_expression(&mut self) {                                         //*
            if self.current_token.token_type == TokenType::IntegerLiteral {              //*
                self.parser_accept_it();                                                //*
            } else if self.current_token.token_type == TokenType::CharLiteral {     //*
                self.parser_accept_it();                                                //*
            } else if self.current_token.token_type == TokenType::Identifier {           //*
                self.parser_accept_it();                                                //*
                self.parse_vname_prime();                                                //*
                if self.current_token.token_type == TokenType::LeftParen {               //*
                    self.parser_accept_it();                                            //*
                    self.parse_formal_parameter_sequence();                               //*
                    self.parser_accept(TokenType::RightParen);                         //*
                }                                                                      //*
            } else if self.current_token.token_type == TokenType::Operator {             //*
                self.parser_accept_it();                                                //*
                self.parse_primary_expression();                                         //*
            } else if self.current_token.token_type == TokenType::LeftParen {            //*
                self.parser_accept_it();                                                //*
                self.parse_expression();                                                //*
                self.parser_accept(TokenType::RightParen);                             //*
            } else if self.current_token.token_type == TokenType::LeftBrace {            //*
                self.parser_accept_it();                                                //*
                self.parse_record_aggregate();                                           //*
                self.parser_accept(TokenType::RightBrace);                             //*
            } else if self.current_token.token_type == TokenType::LeftBracket {          //*
                self.parser_accept_it();                                                //*
                self.parse_array_aggregate();                                            //*
                self.parser_accept(TokenType::RightBracket);                           //*
            }                                                                          //*
        }
        //********************************************************************************

        //  record-Aggregate -> Identifier~Expression rA’
        // rA’-> e|,record-Aggregate
        fn parse_record_aggregate(&mut self) {
            self.parser_accept(TokenType::Identifier);
            self.parser_accept(TokenType::Tilde);
            self.parse_expression();
            self.parse_raprime();
        }

        fn parse_raprime(&mut self) {
            if self.current_token.token_type == TokenType::Comma {
                self.parse_record_aggregate();
            } else {
                //Ok(());
            }
        }

        // array-Aggregate -> expression aA’
        // aA’ -> e |,array-Aggregate

        fn parse_array_aggregate(&mut self) {
            self.parse_expression();
            self.parse_aaprime();
        }

        fn parse_aaprime(&mut self) {
            if self.current_token.token_type == TokenType::Comma {
                self.parse_array_aggregate();
            } else {
                //Ok(());
            }
        }

        // V-name -> identitifer Vn’
        // Vn’ -> identitifer Vn’|[Expression] Vn’|e
        fn parse_vname(&mut self) {
            self.parser_accept(TokenType::Identifier);
            self.parse_vname_prime();
        }

        fn parse_vname_prime(&mut self) {
            if self.current_token.token_type == TokenType::Identifier {
                self.parse_vname();
            } else if self.current_token.token_type == TokenType::LeftBracket {
                self.parser_accept_it();
                self.parse_expression();
                self.parser_accept(TokenType::RightBracket);
                self.parse_vname_prime();
            } else {
                //Ok::<()>(());
            }
        }
    }

fn process_file(input_file: &str, output_file: Option<&str>) -> io::Result<()> {
    let input_path = Path::new(input_file);
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);
    let content = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");

    let mut lexer = Lexer::new(content);

    let output: Box<dyn Write> = if let Some(out_file) = output_file {
        Box::new(File::create(out_file)?)
    } else {
        Box::new(File::create("tokens.out")?)
    };
    let mut output = io::BufWriter::new(output);

    loop {
        let token = lexer.next_token();
        writeln!(output, "Token {{ Tipo: {:?}, Lexema: '{}', Ln: {}, Col: {} }}", token.token_type, token.lexeme, token.row, token.col)?;
        if token.token_type == crate::tokenize::TokenType::EOF {
            break;
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <archivo_entrada> [-o <archivo_salida>]", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let mut output_file: Option<&str> = None;

    if args.len() == 4 && args[2] == "-o" {
        output_file = Some(&args[3]);
    }

    if let Err(e) = process_file(input_file, output_file) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
