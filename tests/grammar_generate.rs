use yatl::generative;

// Only context-free grammar!
// Cannot be left-recursive?
	// Resolving left-recursion: https://en.wikipedia.org/wiki/Left_recursion
// All rules has to have a non-recursive arm?

// 0 => 'f'
// ..
// 45 => '('
//     		|-> ALGEBRAIC
// ..
// 49 => '+'
// 50 => '-'
// ..
// 57 => ')'
// ..
// 255 => 'k'
// 256 => ALGEBRAIC
//      		|-> 45
// 			      |-> 256
// 				        |-> 57
//             |-> 260
//               |-> 256
//                 |-> 57
//           |-> 49
//             |-> 260
//               |-> 256
//                 |-> 57
// 257 => NUMBER
// 258 => HEX
// 259 => WS
// 		|-> 78
// 		|-> 79
// 260 => WS+
// 		|-> 259
// 			|-> 260
// 			|-> 1..77,80..259 (anything other than WS and WS+, i.e. !WS and !WS+)


#[test]
fn generative_can_parse() {
  let grammar = generative! {
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