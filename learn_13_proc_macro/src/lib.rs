// learn_13_proc_macro — custom proc-macros: derive, attribute, function-like.

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



 The 3 KINDS of proc-macro (all live in this crate, all use syn + quote):

   1. DERIVE        -> #[derive(Hello)]        - only ADDS code next to a type (can't modify it)   [above]
   2. ATTRIBUTE     -> #[log_call]             - RECEIVES the item and can REWRITE it              (eg: #[tokio::main] )
   3. FUNCTION-LIKE -> make_answer!()          - expands a name!(...) call into code


*/

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};


/*
 1. DERIVE macro  ->  #[derive(Hello)]

   - Registered with #[proc_macro_derive(Hello)].
   - Takes ONE token stream: `input` = the item it's attached to.
   - Can only ADD code next to the type (an impl block); it CANNOT modify the type itself.
     - eg:  #[derive(Debug)] — it adds a Debug impl and no extra changes in the struct.


   - below example adds...hello method
*/

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

use syn::ItemFn;

/*
 2. ATTRIBUTE macro  ->  #[log_call]

   - Registered with #[proc_macro_attribute].
   - Takes TWO token streams: `attr` = args inside #[...],  `item` = the thing it's attached to.
   - Can REWRITE the whole item. 
     - eg: #[tokio::main] 
       - it takes your `async fn main` and rewrites it into a `fn main` that starts the runtime.
   - Here we keep the same fn but wrap its body with a log line.
*/

#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // PARSE: the item is a whole function -> parse it as an `ItemFn`.
    let func = parse_macro_input!(item as ItemFn);

    // INSPECT: pull out the pieces we want to keep.
    let sig = &func.sig; // fn signature (name, args, return type)
    let body = &func.block; // the { ... } body
    let name = &func.sig.ident; // the fn name

    // BUILD: same signature, body wrapped with a log line.
    quote! {
        #sig {
            println!("--> calling {}", stringify!(#name));
            #body
        }
    }
    .into()
}

/*
    Generated code for  #[log_call] fn greet() { println!("hi"); }  looks like:

        fn greet() {
            println!("--> calling {}", "greet");
            { println!("hi"); }
        }


*/

/*
 3. FUNCTION-LIKE macro  ->  make_answer!()

   - Registered with #[proc_macro].
   - Called like macro_rules! ( name!(...) ), but backed by real Rust code.
   - Here it just defines a function; real ones (e.g. sqlx::query!) 
      - can do arbitrary compile-time work.
*/

#[proc_macro]
pub fn make_answer(_input: TokenStream) -> TokenStream {
    // BUILD: no input to parse, just emit a function.
    quote! {
        fn answer() -> u32 {
            42
        }
    }
    .into()
}

/*
    Generated code for  make_answer!();  looks like:

        fn answer() -> u32 {
            42
        }


*/