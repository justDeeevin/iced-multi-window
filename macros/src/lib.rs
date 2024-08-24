use proc_macro::TokenStream;
use quote::quote;
use std::collections::VecDeque;
use syn::{parse_macro_input, punctuated::Punctuated, Ident, Token};

/// Creates the `WindowUnion` that is used by the library to refer to the different windows.
///
/// `multi_window!` _must be used_ at the top level of your crate.
///
/// Usage:
/// `multi_window!(AppStruct, Window1, Window2, Window3, ...)`
#[proc_macro]
pub fn multi_window(item: TokenStream) -> TokenStream {
    let mut args = parse_macro_input!(item with Punctuated::<Ident, Token![,]>::parse_terminated)
        .into_iter()
        .collect::<VecDeque<_>>();
    let Some(app) = args.pop_front() else {
        panic!("Expected at least two arguments, got 0");
    };
    let windows: Vec<_> = args.into();
    if windows.is_empty() {
        panic!("Expected at least two arguments, got 1");
    }
    quote! {
        #[derive(PartialEq, Eq, Debug, Clone)]
        pub enum WindowUnion {
            #(
                #windows(#windows)
            ),*
        }

        impl iced_multi_window::Window<#app> for WindowUnion {
            fn view<'a>(&'a self, app: &'a #app, ) -> iced::Element<'_, <#app as iced::multi_window::Application>::Message, <#app as iced::multi_window::Application>::Theme> {
                match self {
                    #(
                        Self::#windows(window) => window.view(app)
                    ),*
                }
            }

            fn title(&self, app: &#app, ) -> String {
                match self {
                    #(
                        Self::#windows(window) => window.title(app)
                    ),*
                }
            }

            fn theme(&self, app: &#app, ) -> <#app as iced::multi_window::Application>::Theme {
                match self {
                    #(
                        Self::#windows(window) => window.theme(app)
                    ),*
                }
            }

            fn settings(&self) -> iced::window::Settings {
                match self {
                    #(
                        Self::#windows(window) => window.settings()
                    ),*
                }

            }
        }
    }
    .into()
}

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
