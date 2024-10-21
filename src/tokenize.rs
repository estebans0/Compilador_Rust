use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Assign,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub row: usize,
    pub col: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, row: usize, col: usize) -> Self {
        Token { token_type, lexeme, row, col }
    }
}

pub struct Lexer {
    input: Vec<char>,
    curr_pos: usize,
    next_pos: usize,
    curr_char: char,
    row: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            curr_pos: 0,
            next_pos: 0,
            curr_char: '\0',
            row: 1,
            col: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.curr_char = '\0';
        } else {
            self.curr_char = self.input[self.next_pos];
        }

        if self.curr_char == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }

        self.curr_pos = self.next_pos;
        self.next_pos += 1;
    }

    fn look_ahead(&self) -> char {
        if self.next_pos >= self.input.len() {
            '\0'
        } else {
            self.input[self.next_pos]
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.curr_char.is_whitespace() || self.curr_char == '!' {
            if self.curr_char == '!' {
                while self.curr_char != '\n' && self.curr_char != '\0' {
                    self.read_char();
                }
            }
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start_col = self.col;
        let start_pos = self.curr_pos;

        while is_letter(self.curr_char) || is_digit(self.curr_char) {
            self.read_char();
        }

        let lexeme: String = self.input[start_pos..self.curr_pos].iter().collect();
        let token_type = match lexeme.as_str() {
            "array" => TokenType::Array,
            "begin" => TokenType::Begin,
            "const" => TokenType::Const,
            "do" => TokenType::Do,
            "else" => TokenType::Else,
            "end" => TokenType::End,
            "func" => TokenType::Func,
            "if" => TokenType::If,
            "in" => TokenType::In,
            "let" => TokenType::Let,
            "of" => TokenType::Of,
            "proc" => TokenType::Proc,
            "record" => TokenType::Record,
            "then" => TokenType::Then,
            "type" => TokenType::Type,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        Token::new(token_type, lexeme, self.row, start_col)
    }

    fn read_number(&mut self) -> Token {
        let start_col = self.col;
        let start_pos = self.curr_pos;

        while is_digit(self.curr_char) {
            self.read_char();
        }

        let lexeme: String = self.input[start_pos..self.curr_pos].iter().collect();
        Token::new(TokenType::IntegerLiteral, lexeme, self.row, start_col)
    }

    fn read_operator(&mut self) -> Token {
        let start_col = self.col;
        let start_pos = self.curr_pos;

        while is_operator_char(self.curr_char) {
            self.read_char();
        }

        let lexeme: String = self.input[start_pos..self.curr_pos].iter().collect();
        let token_type = match lexeme.as_str() {
            "+" => TokenType::Operator,
            "-" => TokenType::Operator,
            "*" => TokenType::Operator,
            "/" => TokenType::Operator,
            "/\\" => TokenType::Operator,
            "\\/" => TokenType::Operator,
            "<=" => TokenType::Operator,
            ">=" => TokenType::Operator,
            _ => TokenType::Operator,
        };

        Token::new(token_type, lexeme, self.row, start_col)
    }

    fn read_character(&mut self) -> Token {
        let start_col = self.col;

        self.read_char(); // Skip opening '
        let char_lit = self.curr_char;
        self.read_char(); // Read character
        self.read_char(); // Skip closing '

        Token::new(TokenType::CharLiteral, char_lit.to_string(), self.row, start_col)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        let start_col = self.col;

        let tok = match self.curr_char {
            '{' => self.create_token(TokenType::LeftBrace, start_col),
            '}' => self.create_token(TokenType::RightBrace, start_col),
            '(' => self.create_token(TokenType::LeftParen, start_col),
            ')' => self.create_token(TokenType::RightParen, start_col),
            '[' => self.create_token(TokenType::LeftBracket, start_col),
            ']' => self.create_token(TokenType::RightBracket, start_col),
            ':' => {
                if self.look_ahead() == '=' {
                    self.read_char();
                    Token::new(TokenType::Assign, ":=".to_string(), self.row, start_col)
                } else {
                    self.create_token(TokenType::Colon, start_col)
                }
            }
            ';' => self.create_token(TokenType::Semicolon, start_col),
            ',' => self.create_token(TokenType::Comma, start_col),
            '.' => self.create_token(TokenType::Period, start_col),
            '=' => self.create_token(TokenType::Equals, start_col),
            '~' => self.create_token(TokenType::Tilde, start_col),
            '\'' => {
                return self.read_character();
            }
            '\0' => Token::new(TokenType::EOF, "".to_string(), self.row, start_col),
            _ => {
                if is_letter(self.curr_char) {
                    return self.read_identifier();
                } else if is_digit(self.curr_char) {
                    return self.read_number();
                } else if is_operator_char(self.curr_char) {
                    return self.read_operator();
                } else {
                    self.create_token(TokenType::Illegal, start_col)
                }
            }
        };

        self.read_char();
        tok
    }

    fn create_token(&self, token_type: TokenType, start_col: usize) -> Token {
        Token::new(token_type, self.curr_char.to_string(), self.row, start_col)
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic()
}

fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

fn is_operator_char(ch: char) -> bool {
    matches!(ch, '+' | '-' | '*' | '/' | '=' | '<' | '>' | '&' | '@' | '%' | '^' | '?' | '\\')
}

// Procesar el archivo de input y escribir los tokens en el archivo de output
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
        writeln!(output, "{{{:?}, '{}', {}, {}}}", token.token_type, token.lexeme, token.row, token.col)?;
        if token.token_type == TokenType::EOF {
            break;
        }
    }

    Ok(())
}

#[allow(dead_code)]
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
