extern crate quote;

use quote::quote;

/// Generate the parser for the language
pub fn parser(language: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
  quote! {
    #language
    
    #[derive(Debug)]
    struct Token(&'static str);
  
    fn parse(code: &str) -> Result<Token, String> {
      
      Ok(Token(""))
    }
  }
}