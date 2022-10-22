use yatl::{generative};

#[test]
fn it_works() {
  generative! {
    ALGEBRAIC -> "(" ALGEBRAIC ")"
               | "(" WS+ ALGEBRAIC ")"
               | "(" ALGEBRAIC WS+ ")"
               | "(" WS+ ALGEBRAIC WS+ ")"
               | "+" WS+ ALGEBRAIC
               | "-" WS+ ALGEBRAIC
               | "+" ALGEBRAIC
               | "-" ALGEBRAIC
               | ALGEBRAIC WS+ "/" ALGEBRAIC
               | ALGEBRAIC WS+ "*" ALGEBRAIC
               | ALGEBRAIC "/" WS+ ALGEBRAIC
               | ALGEBRAIC "*" WS+ ALGEBRAIC
               | ALGEBRAIC WS+ "/" WS+ ALGEBRAIC
               | ALGEBRAIC WS+ "*" WS+ ALGEBRAIC
               | ALGEBRAIC "/" ALGEBRAIC
               | ALGEBRAIC "*" ALGEBRAIC
               | ALGEBRAIC WS+ "+" ALGEBRAIC
               | ALGEBRAIC WS+ "-" ALGEBRAIC
               | ALGEBRAIC "+" WS+ ALGEBRAIC
               | ALGEBRAIC "-" WS+ ALGEBRAIC
               | ALGEBRAIC WS+ "+" WS+ ALGEBRAIC
               | ALGEBRAIC WS+ "-" WS+ ALGEBRAIC
               | ALGEBRAIC "+" ALGEBRAIC
               | ALGEBRAIC "-" ALGEBRAIC
               | NUMBER;
    NUMBER -> DECIMAL | FLOAT | HEX;
    HEX -> "0x" DIGIT+;
    FLOAT -> DIGIT+ "." DIGIT+;
    DECIMAL -> DIGIT+;
    DIGIT -> "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
    WS -> " " | HT | LF | CR;
    LF -> b10;
    CR -> b13;
    HT -> b9;
  };

  // IDENT -> LETTER (LETTER | DIGIT)*
  // LETTER -> "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "x" | "y" | "z" | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "X" | "Y" | "Z"

  
  /*match parse("3 +5/3") {
    Err(s) => println!("ERROR: {}", s),
    Ok(x) => println!("{:#?}", x),
  };*/
}
