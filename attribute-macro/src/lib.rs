extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn my_macro(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", _metadata.to_string());
    println!("item: \"{}\"", _input.to_string());
    _input
}
