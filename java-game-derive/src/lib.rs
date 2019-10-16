extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{export::Span, parse_macro_input, Attribute, DeriveInput, Ident};

#[proc_macro_derive(Positionable, attributes(PosField))]
pub fn positionable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let pos_field = parse_pos_field(input.attrs);

    let gen = quote! {
        impl Positionable for #name {
            fn pos(&self) -> Vec2<u8> { self.#pos_field }
            fn set_pos(&mut self, pos: Vec2<u8>) { self.#pos_field = pos }
        }
    };

    gen.into()
}

fn parse_pos_field(attrs: Vec<Attribute>) -> Ident {
    // TODO: Actually parse the ident instead of always using pos
    Ident::new("pos", Span::call_site())
}
