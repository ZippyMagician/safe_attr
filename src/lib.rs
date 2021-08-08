//! # safe_attr
//! safe_attr provides a `#[safe]` attribute to mark functions with
//!
//! Take this example code:
//! ```
//! union Num {
//!     float: f32,
//!     long: u32,
//! }
//! 
//! fn use_both() {
//!     let mut num = Num { long: 132 };
//!     let the_float = unsafe { num.float };
//!     // do things with the float...
//!     let the_long = unsafe { num.long };
//!     // do things with the long...
//! 
//!     // maybe use some more unsafes later...
//! }
//! ```
//! Now, you know this is perfectly safe. Both of the types are of the same size, converting it shouldn't require such verbosity. With `safe_attr`, you can now do:
//! ```
//! use safe_attr::safe;
//! 
//! #[safe]
//! fn use_both() {
//!     let mut num = Num { long: 132 };
//!     let the_float = num.float;
//!     let the_long = num.long;
//! }
//! ```
//! This attribute allows you to avoid spamming `unsafe`s everywhere in code that doesn't require it. 
//! 
//! This **does not** mean this attribute should be abused. It could easily lead to making unsafe code's bugs harder to track down in larger functions, and also simply makes it harder to find problem spots. As such, you are encouraged to still mark the function with a `// Safety:` comment, and furthermore only use this attribute for use cases *similar* to the above example.

#[macro_use]
extern crate quote;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, TokenTree};

#[proc_macro_attribute]
pub fn safe(_: TokenStream, body: TokenStream) -> TokenStream {
    let stream: proc_macro2::TokenStream = body.into();
    let mut found_body = false;
    let mut construct = Vec::new();

    for tok in stream {
        if let TokenTree::Group(group) = &tok {
            if group.delimiter() == Delimiter::Brace && !found_body {
                found_body = true;
                let span = group.stream();
                construct.push(quote! { { unsafe { #span } } }.into_iter().next().unwrap());
            } else {
                construct.push(tok);
            }
        } else {
            construct.push(tok);
        }

        if found_body {
            break;
        }
    }

    let mut stream = proc_macro2::TokenStream::new();
    stream.extend(construct);
    TokenStream::from(stream)
}
