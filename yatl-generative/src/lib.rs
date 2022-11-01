#![doc = include_str!("../README.md")]

extern crate yatl_common;
extern crate quote;

use yatl_common::grammar::{Error, TokenId, Token};
use quote::{quote, quote_spanned};


#[derive(PartialEq)]
enum State {
  Expression,
  Definition1,
  Definition2,
  Rule,
}

#[derive(Default, Debug)]
struct Tokens {
  data: Vec<Token>,
}

impl Tokens {
  fn add(self: &mut Self, token: Token) -> TokenId {
    self.data.push(token);
    self.data.len() - 1
  }

  fn get_or_add_id(self: &mut Self, token: Token) -> TokenId {
    self.data
      .iter()
      .position(|x| token.eq(x))
      .unwrap_or_else(|| self.add(token))
  }

  fn get_token(self: &Self, token_id: TokenId) -> Option<&Token> {
    self.data.get(token_id)
  }
}

#[derive(Debug)]
struct Rule {
  ident: TokenId,
  definition: Vec<TokenId>,
}

impl Rule {
  fn new(ident: TokenId) -> Self {
    Rule {
      ident: ident,
      definition: Vec::new(),
    }
  }
}

type Rules = Vec<Rule>;

fn parse(input: proc_macro2::TokenStream) -> Result<(Tokens, Rules), Error> {
  let mut tokens = Tokens::default();
  let mut rules = Rules::default();
  let mut state = State::Expression;
  let mut identifier: Option<TokenId> = None;
  let mut rule: Option<Rule> = None;

  // Start parsing the generative grammar
  for tt in input.into_iter() {
    match &tt {
      // Left side of a definition, so recording the identifier this ruleset refers to
      proc_macro2::TokenTree::Ident(ident) if state == State::Expression => {
        let name = ident.to_string();
        let token = Token::Ident(name);
        let token_id = tokens.get_or_add_id(token);
        identifier = Some(token_id);
        state = State::Definition1;
      },
      proc_macro2::TokenTree::Punct(ch) if ch.as_char() == '-' && state == State::Definition1 => {
        state = State::Definition2;
      },
      proc_macro2::TokenTree::Punct(ch) if ch.as_char() == '>' && state == State::Definition2 => {
        if let Some(id) = identifier {
          rule = Some(Rule::new(id));
          state = State::Rule;
        } else {
          return Err(Error {
            token: tt.clone(),
            msg: format!("Expected an expression name (i.e. [a-zA-Z][a-zA-Z0-9]*). Got {}", tt.to_string()),
          });
        }
      },

      // Process the end of a ruleset
      proc_macro2::TokenTree::Punct(ch) if ch.as_char() == ';' && state == State::Rule => {
        state = State::Expression;
      },

      // Break out the rulesets into rules
      proc_macro2::TokenTree::Punct(ch) if ch.as_char() == '|' => {
        if let Some(id) = identifier {
          rules.push(rule.unwrap());
          rule = Some(Rule::new(id));
          state = State::Rule;
        } else {
          return Err(Error {
            token: tt.clone(),
            msg: format!("Expected an expression name (i.e. [a-zA-Z][a-zA-Z0-9]*). Got {}", tt.to_string()),
          });
        }
      },

      // Collecting the elements of the rule
      proc_macro2::TokenTree::Ident(ident) if state == State::Rule => {
        let identifier = ident.to_string();
        let token_id = tokens.get_or_add_id(Token::Ident(identifier));
        if let Some(target) = rule.as_mut() {
          target.definition.push(token_id);
          state = State::Rule;
        } else {
          return Err(Error {
            token: tt.clone(),
            msg: format!("Expected an expression name. Got {}", tt.to_string()),
          });
        }
      },
      proc_macro2::TokenTree::Literal(ll) => {
        let literal = match (
          ll.to_string().as_bytes(),
          ll.to_string().parse::<u8>(),
        ) {
          (_, Ok(byte)) => vec![byte], // If byte literal is given, use raw byte
          ([b'"', content @ .., b'"'], _) => content.to_vec(), // If quoted on both both sides, it's a string
          _ => {
            return Err(Error {
              token: tt.clone(),
              msg: format!("Expected a string literal or a raw byte code. Got {}", tt.to_string()),
            });
          },
        };
        
        if let Some(target) = rule.as_mut() {
          target.definition.push(tokens.get_or_add_id(Token::Literal(literal)));
          state = State::Rule;
        } else {
          return Err(Error {
            token: tt.clone(),
            msg: format!("Expected an expression name. Got {}", tt.to_string()),
          });
        }
      },
      proc_macro2::TokenTree::Punct(ch) if ch.as_char() == '+' => {
        if let Some(target) = rule.as_mut() {
          if let Some(old_token_id) = target.definition.pop() {
            if let Some(old_token) = tokens.get_token(old_token_id) {
              let new_token = match old_token {
                Token::Ident(x) => Token::Ident(String::from_iter([x, ch.to_string().as_str()])),
                Token::Literal(x) => Token::Literal([&x[..], &String::from(ch.as_char()).into_bytes()[..]].concat()),
              };
              let new_token_id = tokens.get_or_add_id(new_token);
              target.definition.push(new_token_id);
              state = State::Rule;
            }
          } else {
            return Err(Error {
              token: tt.clone(),
              msg: format!("Expected an expression name, not a rule definition. Got {}", tt.to_string()),
            });
          }
        } else {
          return Err(Error {
            token: tt.clone(),
            msg: format!("Expected an expression name. Got {}", tt.to_string()),
          });
        }
      }

      // Handle failed parse
      _ => {
        return Err(
          match state {
            State::Expression =>
              Error {
                token: tt.clone(),
                msg: format!("Expression name expected (i.e. [a-zA-Z0-9]+). Got: {}\n", tt.to_string()),
              },
            State::Definition1 => 
              Error {
                token: tt.clone(),
                msg: format!("Expected rule definition operator '->'. Got: {}\n", tt.to_string()),
              },
            State::Definition2 => 
              Error {
                token: tt.clone(),
                msg: format!("Expected rule definition operator '->'. Got: {}\n", tt.to_string()),
              },
            State::Rule => 
              Error {
                token: tt.clone(),
                msg: format!(
                      concat!("Expected an expression reference (i.e. [a-zA-Z0-9]+), a literal (i.e. \".+\"), ",
                              "a rule separator (i.e. '|') or definition closure (i.e. ';'). Got: {}\n"), 
                      tt.to_string()),
              },
          }
        );
      },
    }
  }

  Ok((tokens, rules))
}

/// Generate grammar from the "formal" definition description
/// 
/// Rules you can use:
/// SOME_EXPR -> "=" OTHER_EXPR
/// OTHER_EXPR -> "0"
pub fn generative(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
  match parse(input) {
    Ok((tokens, rules)) => {
      println!("{:#?}", tokens);
      println!("{:#?}", rules);
      //tokens: [#(#toks),*],
      quote! {
        Vec::<u8>::new()
      }
    },
    Err(e) => {
      let msg = e.msg;
      quote_spanned! {
        e.token.span().into() =>
          compile_error!(#msg)
      }
    }
  }
}