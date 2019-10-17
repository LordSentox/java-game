extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    export::Span, parse_macro_input, Attribute, DeriveInput, Ident, Lit, Meta, MetaNameValue
};

#[proc_macro_derive(Positionable, attributes(PosField))]
pub fn positionable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let pos_field = parse_pos_field(input.attrs);

    // TODO: Check if the struct contains the field specified, to give better error
    // TODO  messages

    let gen = quote! {
        use crate::map::FieldPos;

        impl Positionable for #name {
            fn pos(&self) -> FieldPos { self.#pos_field }
            fn set_pos(&mut self, pos: FieldPos) { self.#pos_field = pos }
        }
    };

    gen.into()
}

fn parse_pos_field(attrs: Vec<Attribute>) -> Ident {
    for attr in attrs.iter() {
        if attr.path.is_ident("PosField") {
            match attr.parse_meta() {
                Ok(Meta::NameValue(MetaNameValue {
                    lit: Lit::Str(pos_field),
                    ..
                })) => {
                    return Ident::new(&pos_field.value(), Span::call_site());
                }
                _ => {}
            }
        }
    }

    Ident::new("pos", Span::call_site())
}
