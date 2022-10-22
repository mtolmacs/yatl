extern crate proc_macro2;

use quote::quote;


/// Generate the parser for the language
pub fn yatl(language: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
  quote! {
    #language
    
    #[derive(Debug)]
    struct Token(&'static str);
  
    fn parse(code: &str) -> Result<Token, String> {
      
      Ok(Token(""))
    }
  }
}

/*
use std::io::Read;

pub struct Parser<'a, T> {
  root: State<'a, T>,
  reader: &'a mut dyn Read,
}

impl<'a, T> Parser<'a, T> {
  pub fn new(root: State<'a, T>, reader: &'a mut dyn Read) -> Parser<'a, T> {
    Parser {
      root,
      reader,
    }
  }

  #[inline(always)]
  fn next<R>(self, emitter: &dyn Fn(T, u8) -> R) -> Result<R, String> {
    let mut current_state = &self.root;
    let mut content: Vec<u8> = Vec::new();
    let mut bytes = self.reader.bytes();

    while let Some(byte) = bytes.next() {
      if let Err(error) = byte {
        // If there was an error reading the next byte, then we bail
        return Err(format!("Error reading source code: {:#?}", error));
      }

      let ch = byte.unwrap();
      match current_state.edges.0[ch as usize] {
        Some(state) => {
          content.push(ch);
          match &state.payload {
            None => current_state = state,
            Some(t) => {
              //return Ok(emitter(t, content.as_slice()));
              
            },
          }
        },
        None => return Err("No next rule to evaluate".to_string()),
      }
    }

    Err("Terminated early".to_string())
  }
}
*/

/*
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    enum Token {
      NoOp(Vec<u8>),
    }

    fn emit(payload: &[u8]) -> Token {
      Token::NoOp(Vec::from(payload))
    }
    
    const root: LookupTable<&State<emit, Token>> = LookupTable([
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None, None, None, None, None,
      None, None, None, None, None, None
    ]);
    
    return ()
  }
}
*/