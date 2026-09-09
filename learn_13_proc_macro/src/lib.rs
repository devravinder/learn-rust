// learn_13_proc_macro — implementing a custom #[derive(Hello)] macro.

/*

 PROCEDURAL Macros ( proc-macro ):-
   - macros that run at compile time ( and transforms a token stream )


   - The flow of every derive macro is the same three steps:
        1. PARSE  the incoming tokens into a syntax tree           - (syn)
        2. INSPECT the tree to learn what we need                  - (e.g. the name)
        3. BUILD  new tokens (the generated code) and hand back    - (quote)


   - eg: 
      - built-in: #[derive(Debug)]  
      - our custom: #[drive(Hello)]

     Note:- The word `derive` in #[derive(Hello)] is the compiler's built-in attribute (dispatcher ) that invokes the proc-macro — it is NOT the macro name.


*/

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

// `#[proc_macro_derive(Hello)]` registers the name ...so we can use like #[derive(Hello)].
    // The fn name (`derive_hello`) is internal — callers never see it.

#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    // 1. PARSE: the raw tokens of the annotated item into a structured `DeriveInput` (holds the name, generics, fields, etc.).
    let ast = parse_macro_input!(input as DeriveInput);

    // 2. INSPECT: grab the identifier (the struct/enum name).
    let name = &ast.ident;

    // 3. BUILD: `quote!` is a template that produces Rust tokens.
    let generated = quote! {
        impl #name {
            fn hello(&self) {
                println!("Hello from {}!", stringify!(#name));
            }
        }
    };
    // Hand the generated tokens back to the compiler.
    generated.into()
}

/*
    Generated code for  #[derive(Hello)] struct Dog;  looks like:
    
        impl Dog {
            fn hello(&self) {
                println!("Hello from {}!", "Dog");
            }
        }


*/