use proc_macro::TokenStream;
use quote::quote;
use syn;

/// * "The hello_macro_derive function will be called when a user of our library
///   specifies #[derive(HelloMacro)] on a type.""
///
/// * "This is possible because we’ve annotated the hello_macro_derive function
///   here with proc_macro_derive and specified the name HelloMacro, which matches
///   our trait name;""
///
/// * "This is the convention most procedural macros follow!""
/// 
/// * "Note that the output for our derive macro is also a TokenStream.""
/// 
/// * "The returned TokenStream is added to the code that our crate users write, so
///   when they compile their crate, they’ll get the extra functionality that we
///   provide in the modified TokenStream.""
///
/// * "We’re calling unwrap to cause the hello_macro_derive function to panic if
///   the call to the syn::parse function fails here.""
///
/// * "It’s necessary for our procedural macro to panic on errors because
///   proc_macro_derive functions must return TokenStream rather than Result to
///   conform to the procedural macro API.""
///
/// * "We’ve simplified this example by using unwrap; in production code, you
///   should provide more specific error messages about what went wrong by using
///   panic! or expect.""
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast : &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}.", stringify!(#name));
            }
        }
    };

    gen.into()
}