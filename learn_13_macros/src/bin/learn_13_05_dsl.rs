// learn_13_05_dsl — a tiny Domain Specific Language via macros.
// Run: cargo run --bin learn_13_05_dsl

/*
  DSL ( Domain-specific languages ):-
   - as macros match on syntax, you can invent your own mini-language
      - this will expand into normal Rust code
   - 

*/

macro_rules! calculate {
    (eval $e:expr) => {{
        let val: usize = $e; // force the type of the expression
        println!("{} = {}", stringify!($e), val);
    }};


    // Decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{

        // the inner square brackets {} is to return entire block code 

        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};

}

fn main() {
    calculate!(eval 1 + 2);
    calculate!(eval (1 + 2) * (3 / 4));
    calculate!(eval ((1 + 2) * 3) + 100);

    println!("================");
    
    calculate! {
        eval 1 + 2 
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }

    println!("====================");

    calculate! { 
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }

}
