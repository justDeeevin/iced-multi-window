use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Takes in a [`Window`] identifier and returns an enum variant that can be used by various
/// functions.
#[proc_macro]
pub fn window(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::ExprStruct);

    let ident = item.path.get_ident().unwrap();

    quote! {
        crate::WindowUnion::#ident(#item)
    }
    .into()
}
