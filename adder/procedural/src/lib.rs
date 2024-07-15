use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_macro_input, Error, ExprTuple};

#[proc_macro]
pub fn adder_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExprTuple);
    if input.elems.len() != 2 {
        return Error::new(Span::call_site(), "Expected a tuple with two elements")
            .to_compile_error()
            .into();
    }
    let arg0 = &input.elems[0];
    let arg1 = &input.elems[1];
    let submod = generate_crate_access();
    quote! {
        #submod::add(#arg0, #arg1)
    }
    .into()
}

fn generate_crate_access() -> TokenStream2 {
    match crate_name("adder") {
        // Doesn't work for test in `adder` crate if we use
        // Ok(FoundCrate::Itself) => quote! { adder::submod },
        Ok(FoundCrate::Itself) => quote! { crate::submod },
        Ok(FoundCrate::Name(name)) => {
            // This is the case when the crate is being compiled as a proc-macro
            // but the crate name is different from "test-proc-macro".
            // In this case, we can use the crate's public API.
            // ...
            let name = syn::Ident::new(&name, Span::call_site());
            quote! { #name::submod }
        }
        Err(e) => Error::new(Span::call_site(), e).to_compile_error().into(),
    }
}
