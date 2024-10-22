use std::fs::File;
use std::io::{BufRead, Write, BufReader};
use std::env;

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
    Assign,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxError {
    UnexpectedToken {
        expected: TokenType,
        found: TokenType,
        row: usize,
        col: usize,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub row: usize,
    pub col: usize,
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, row: usize, col: usize) -> Self {
        Token {
            token_type,
            lexeme,
            row,
            col,
        }
    }
}

#[derive(Debug)]
pub enum ASTNode {
    Let(Box<ASTNode>, Box<ASTNode>),
    Const(String, Box<ASTNode>),
    Var(String, String),
    Func(String, Vec<ASTNode>, String, Box<ASTNode>),
    Proc(String, Vec<ASTNode>, Box<ASTNode>),
    Type(String, Box<ASTNode>),
    Assign(Vec<String>, Box<ASTNode>),
    If(Box<ASTNode>, crate::TokenType , Box<ASTNode>, crate::TokenType ,Box<ASTNode>),
    While(Box<ASTNode>, Box<ASTNode>),
    Call(String, Vec<ASTNode>),
    Expression(Box<ASTNode>),
    Identifier(String),
    Number(i64),
    Char(char),
    Operator(String, Box<ASTNode>, Box<ASTNode>),
    Declaration(Vec<ASTNode>),
    Command(Vec<ASTNode>),
}

pub struct Parser {
  pub current_token: Token,
  pub tokens: Vec<Token>,
  pub index: usize,
}
impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
      let current_token = tokens[0].clone();
      Parser {
          current_token,
          tokens,
          index: 0,
      }
  }

  fn next_token(&mut self) {
    if self.index < self.tokens.len() - 1 {
        self.index += 1;
        self.current_token = self.tokens[self.index].clone();
    }
  }

  fn accept(&mut self, expected: TokenType) -> Result<(), SyntaxError> {
      if self.current_token.token_type == expected {
          self.next_token();
          Ok(())
      } else {
          Err(SyntaxError::UnexpectedToken {
              expected,
              found: self.current_token.token_type.clone(),
              row: self.current_token.row,
              col: self.current_token.col,
          })
      }
  }

  pub fn parse(&mut self) -> Result<ASTNode, SyntaxError> {
      self.parse_command()
  }

  fn parse_command(&mut self) -> Result<ASTNode, SyntaxError> {
      let command = self.parse_single_command()?;
      let mut commands = vec![command];

      while self.current_token.token_type == TokenType::Semicolon {
          self.next_token();
          commands.push(self.parse_single_command()?);
      }

      if commands.len() == 1 {
          Ok(commands.pop().unwrap())
      } else {
          Ok(ASTNode::Command(commands))
      }
  }

  fn parse_single_command(&mut self) -> Result<ASTNode, SyntaxError> {
      match self.current_token.token_type {
          TokenType::Let => {
              self.next_token();
              let decl = self.parse_declaration_sequence()?;
              self.accept(TokenType::In)?;
              let cmd = self.parse_command()?;
              Ok(ASTNode::Let(Box::new(decl), Box::new(cmd)))
          }
          TokenType::Const => {
              self.next_token();
              let name = self.parse_identifier()?;
              self.accept(TokenType::Tilde)?;
              let expr = self.parse_expression()?;
              Ok(ASTNode::Const(name, Box::new(expr)))
          }
          TokenType::Var => {
              self.next_token();
              let name = self.parse_identifier()?;
              self.accept(TokenType::Colon)?;
              let type_name = self.parse_identifier()?;
              Ok(ASTNode::Var(name, type_name))
          }
          TokenType::Func => {
              self.next_token();
              let name = self.parse_identifier()?;
              self.accept(TokenType::LeftParen)?;
              let params = self.parse_formal_parameter_sequence()?;
              self.accept(TokenType::RightParen)?;
              self.accept(TokenType::Colon)?;
              let return_type = self.parse_identifier()?;
              self.accept(TokenType::Tilde)?;
              let body = self.parse_expression()?;
              Ok(ASTNode::Func(name, params, return_type, Box::new(body)))
          }
          TokenType::If => {
              self.next_token();
              let condition = self.parse_expression()?;
              self.accept(TokenType::Then)?;
              let then_branch = self.parse_command()?;
              self.accept(TokenType::Else)?;
              let else_branch = self.parse_command()?;
              Ok(ASTNode::If(
                  Box::new(condition),
                  TokenType::Then,
                  Box::new(then_branch),
                  TokenType::Else,
                  Box::new(else_branch),
              ))
          }
          TokenType::While => {
              self.next_token();
              let condition = self.parse_expression()?;
              self.accept(TokenType::Do)?;
              let body = self.parse_command()?;
              Ok(ASTNode::While(Box::new(condition), Box::new(body)))
          }
          TokenType::Begin => {
              // Soporte para comandos 'begin ... end'
              self.next_token();
              let commands = self.parse_command()?;
              self.accept(TokenType::End)?;
              Ok(commands)
          }
          TokenType::Identifier => {
              let name = self.parse_identifier()?;
              if self.current_token.token_type == TokenType::Assign {
                  self.next_token();
                  let expr = self.parse_expression()?;
                  Ok(ASTNode::Assign(vec![name], Box::new(expr)))
              } else if self.current_token.token_type == TokenType::LeftParen {
                  self.next_token();
                  let params = self.parse_actual_parameter_sequence()?;
                  self.accept(TokenType::RightParen)?;
                  Ok(ASTNode::Call(name, params))
              } else {
                  Err(SyntaxError::UnexpectedToken {
                      expected: TokenType::Assign,
                      found: self.current_token.token_type.clone(),
                      row: self.current_token.row,
                      col: self.current_token.col,
                  })
              }
          }
          _ => Err(SyntaxError::UnexpectedToken {
              expected: TokenType::Identifier,
              found: self.current_token.token_type.clone(),
              row: self.current_token.row,
              col: self.current_token.col,
          }),
      }
  }

  fn parse_expression(&mut self) -> Result<ASTNode, SyntaxError> {
      match self.current_token.token_type {
          TokenType::Let => {
              self.next_token();
              let decls = self.parse_declaration_sequence()?;
              self.accept(TokenType::In)?;
              let expr = self.parse_expression()?;
              Ok(ASTNode::Let(Box::new(decls), Box::new(expr)))
          }
          TokenType::If => {
              self.next_token();
              let condition = self.parse_expression()?;
              self.accept(TokenType::Then)?;
              let then_branch = self.parse_expression()?;
              self.accept(TokenType::Else)?;
              let else_branch = self.parse_expression()?;
              Ok(ASTNode::If(
                  Box::new(condition),
                  TokenType::Then,
                  Box::new(then_branch),
                  TokenType::Else,
                  Box::new(else_branch),
              ))
          }
          _ => self.parse_second_expression(),
      }
  }

  fn parse_declaration_sequence(&mut self) -> Result<ASTNode, SyntaxError> {
      let mut declarations = vec![self.parse_single_declaration()?];

      while self.current_token.token_type == TokenType::Semicolon {
          self.next_token();
          declarations.push(self.parse_single_declaration()?);
      }

      if declarations.len() == 1 {
          Ok(declarations.pop().unwrap())
      } else {
          Ok(ASTNode::Declaration(declarations))
      }
  }

  fn parse_single_declaration(&mut self) -> Result<ASTNode, SyntaxError> {
      match self.current_token.token_type {
          TokenType::Const => {
              self.next_token();
              let name = self.parse_identifier()?;
              self.accept(TokenType::Tilde)?;
              let expr = self.parse_expression()?;
              Ok(ASTNode::Const(name, Box::new(expr)))
          }
          TokenType::Var => {
              self.next_token();
              let name = self.parse_identifier()?;
              self.accept(TokenType::Colon)?;
              let type_name = self.parse_identifier()?;
              Ok(ASTNode::Var(name, type_name))
          }
          TokenType::Func => {
              self.next_token();
              let name = self.parse_identifier()?;
              self.accept(TokenType::LeftParen)?;
              let params = self.parse_formal_parameter_sequence()?;
              self.accept(TokenType::RightParen)?;
              self.accept(TokenType::Colon)?;
              let return_type = self.parse_identifier()?;
              self.accept(TokenType::Tilde)?;
              let body = self.parse_expression()?;
              Ok(ASTNode::Func(name, params, return_type, Box::new(body)))
          }
          _ => Err(SyntaxError::UnexpectedToken {
              expected: TokenType::Const,
              found: self.current_token.token_type.clone(),
              row: self.current_token.row,
              col: self.current_token.col,
          }),
      }
  }

  fn parse_primary_expression(&mut self) -> Result<ASTNode, SyntaxError> {
      match self.current_token.token_type {
          TokenType::IntegerLiteral => {
              let value = self.current_token.lexeme.parse::<i64>().unwrap();
              self.next_token();
              Ok(ASTNode::Number(value))
          }
          TokenType::CharLiteral => {
              let value = self.current_token.lexeme.chars().next().unwrap();
              self.next_token();
              Ok(ASTNode::Char(value))
          }
          TokenType::Identifier => {
              let name = self.parse_identifier()?;
              if self.current_token.token_type == TokenType::LeftParen {
                  self.next_token();
                  let params = self.parse_actual_parameter_sequence()?;
                  self.accept(TokenType::RightParen)?;
                  Ok(ASTNode::Call(name, params))
              } else {
                  Ok(ASTNode::Identifier(name))
              }
          }
          TokenType::LeftParen => {
              self.next_token();
              let expr = self.parse_expression()?;
              self.accept(TokenType::RightParen)?;
              Ok(ASTNode::Expression(Box::new(expr)))
          }
          _ => Err(SyntaxError::UnexpectedToken {
              expected: TokenType::Identifier,
              found: self.current_token.token_type.clone(),
              row: self.current_token.row,
              col: self.current_token.col,
          }),
      }
  }

  fn parse_second_expression(&mut self) -> Result<ASTNode, SyntaxError> {
      let primary = self.parse_primary_expression()?;
      self.parse_second_expression_prime(primary)
  }

  fn parse_second_expression_prime(
      &mut self,
      left: ASTNode,
  ) -> Result<ASTNode, SyntaxError> {
      if self.current_token.token_type == TokenType::Operator {
          let op = self.current_token.lexeme.clone();
          self.next_token();
          let right = self.parse_primary_expression()?;
          let expr = ASTNode::Operator(op, Box::new(left), Box::new(right));
          self.parse_second_expression_prime(expr)
      } else {
          Ok(left)
      }
  }

  fn parse_identifier(&mut self) -> Result<String, SyntaxError> {
      if let TokenType::Identifier = self.current_token.token_type {
          let name = self.current_token.lexeme.clone();
          self.next_token();
          Ok(name)
      } else {
          Err(SyntaxError::UnexpectedToken {
              expected: TokenType::Identifier,
              found: self.current_token.token_type.clone(),
              row: self.current_token.row,
              col: self.current_token.col,
          })
      }
  }

  fn parse_formal_parameter_sequence(&mut self) -> Result<Vec<ASTNode>, SyntaxError> {
      let mut params = Vec::new();
      if self.current_token.token_type != TokenType::RightParen {
          params.push(self.parse_formal_parameter()?);
          while self.current_token.token_type == TokenType::Comma {
              self.next_token();
              params.push(self.parse_formal_parameter()?);
          }
      }
      Ok(params)
  }

  fn parse_formal_parameter(&mut self) -> Result<ASTNode, SyntaxError> {
      if self.current_token.token_type == TokenType::Var {
          self.next_token();
          let name = self.parse_identifier()?;
          self.accept(TokenType::Colon)?;
          let type_name = self.parse_identifier()?;
          Ok(ASTNode::Var(name, type_name))
      } else {
          let name = self.parse_identifier()?;
          self.accept(TokenType::Colon)?;
          let type_name = self.parse_identifier()?;
          Ok(ASTNode::Var(name, type_name))
      }
  }

  fn parse_actual_parameter_sequence(&mut self) -> Result<Vec<ASTNode>, SyntaxError> {
      let mut params = Vec::new();
      if self.current_token.token_type != TokenType::RightParen {
          params.push(self.parse_expression()?);
          while self.current_token.token_type == TokenType::Comma {
              self.next_token();
              params.push(self.parse_expression()?);
          }
      }
      Ok(params)
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
      eprintln!("Usage: parse <input_file> [-o <output_file>]");
      std::process::exit(1);
  }

  let input_file = &args[1];
  let output_file = if args.len() > 3 && args[2] == "-o" {
      &args[3]
  } else {
      "tree.out"
  };

  let file = File::open(input_file).expect("Unable to open input file");
  let reader = BufReader::new(file);

  let mut tokens = Vec::new();
  for line in reader.lines() {
      let line = line.expect("Unable to read line");
      let parts: Vec<&str> = line.split(',').collect();
      if parts.len() >= 4 {
          let token_type_str = parts[0].trim_matches('{').trim();
          let lexeme = parts[1].trim().trim_matches('\'').to_string();
          let row: usize = parts[2].trim().parse().expect("Invalid row number");
          let col: usize = parts[3].trim_matches('}').trim().parse().expect("Invalid column number");
          let token_type = match token_type_str {
              "EOF" => TokenType::EOF,
              "Illegal" => TokenType::Illegal,
              "Identifier" => TokenType::Identifier,
              "IntegerLiteral" => TokenType::IntegerLiteral,
              "CharLiteral" => TokenType::CharLiteral,
              "Operator" => TokenType::Operator,
              "Array" => TokenType::Array,
              "Begin" => TokenType::Begin,
              "Const" => TokenType::Const,
              "Do" => TokenType::Do,
              "Else" => TokenType::Else,
              "End" => TokenType::End,
              "Func" => TokenType::Func,
              "If" => TokenType::If,
              "In" => TokenType::In,
              "Let" => TokenType::Let,
              "Of" => TokenType::Of,
              "Proc" => TokenType::Proc,
              "Record" => TokenType::Record,
              "Then" => TokenType::Then,
              "Type" => TokenType::Type,
              "Var" => TokenType::Var,
              "While" => TokenType::While,
              "Period" => TokenType::Period,
              "Colon" => TokenType::Colon,
              "Semicolon" => TokenType::Semicolon,
              "Comma" => TokenType::Comma,
              "Equals" => TokenType::Equals,
              "Tilde" => TokenType::Tilde,
              "LeftParen" => TokenType::LeftParen,
              "RightParen" => TokenType::RightParen,
              "LeftBracket" => TokenType::LeftBracket,
              "RightBracket" => TokenType::RightBracket,
              "LeftBrace" => TokenType::LeftBrace,
              "RightBrace" => TokenType::RightBrace,
              "Assign" => TokenType::Assign,
              _ => {
                  eprintln!("Invalid token type: {}", token_type_str);
                  std::process::exit(1);
              }
          };
          tokens.push(Token::new(token_type, lexeme, row, col));
      }
  }
  let mut parser = Parser::new(tokens);
  let ast = parser.parse();
  match ast {
      Ok(ast) => {
          let mut output = File::create(output_file).expect("Unable to create output file");
          write!(output, "{:#?}", ast).expect("Unable to write to output file");
      }
      Err(err) => {
          eprintln!(
              "Error at row {}, col {}: {:?}",
              parser.current_token.row, parser.current_token.col, err
          );
          std::process::exit(1);
      }
  }
}