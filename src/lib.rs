use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let day;
    let (input_path, example_path) = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(lit_day))] => {
            day = lit_day.token().to_string();
            (
                format!("../../inputs/{}.in", day),
                format!("../../input_examples/{}.in", day),
            )
        }
        _ => panic!("Expected one integer argument"),
    };

    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
      /// Returns `actual` when using real problem input, and `example` when using example input
      fn phi<T>(actual: T, example: T) -> T {
        let args: Vec<String> = std::env::args().collect();
        let mut example_input = false;
        if let Some(a) = args.get(1) {
          if a == "example" {
            example_input = true;
          }
        }
        if example_input {
          example
        } else {
          actual
        }
      }

      const INPUT: &str = include_str!(#input_path);
      const EXAMPLE_INPUT: &str = include_str!(#example_path);
      #aoc_solution
      fn main() {
        let args: Vec<String> = std::env::args().collect();
        let mut example_input = false;
        let mut input = INPUT;
        if let Some(a) = args.get(1) {
          if a == "example" {
            example_input = true;
            input = EXAMPLE_INPUT;
          }
        }

        println!("\x1b[4;1mDay {}:\x1b[0m", #day);
        let now = ::std::time::Instant::now();
        let (p1, p2) = aoc_solution(input.trim_end());
        let time = now.elapsed();

        let ans1 = std::fs::read_to_string(format!("answers/{}p1.sol", #day)).expect("unable to find answer file");
        let ans2 = std::fs::read_to_string(format!("answers/{}p2.sol", #day)).expect("unable to find answer file");

        print!("Part one: ");
        if ans1 != "" && !example_input {
          if ans1 == p1.to_string() {
            print!("\x1b[32m");
          } else {
            print!("\x1b[31m");
          }
        }
        println!("{}\x1b[0m", p1);

        print!("Part two: ");
        if ans2 != "" && !example_input {
          if ans2 == p2.to_string() {
            print!("\x1b[32m");
          } else {
            print!("\x1b[31m");
          }
        }
        println!("{}\x1b[0m", p2);
        if !example_input {
          if time.as_millis() <= 10 {
            print!("\x1b[102m"); // green
          } else if time.as_millis() <= 1000 {
            print!("\x1b[103m"); // yellow
          } else {
            print!("\x1b[101m"); // red
          }
          print!("\x1b[30m");
        }
        if time.as_millis() > 0 {
          print!("Time: {}ms", time.as_millis());
        } else {
          print!("Time: {}μs", time.as_micros());
        }
        println!("\x1b[0m");
      }
    };
    TokenStream::from(tokens)
}
