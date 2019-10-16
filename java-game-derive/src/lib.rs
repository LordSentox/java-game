extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Positionable, attributes(PosField))]
pub fn positionable_derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let data = add_pos_field(input.data);

    let name = input.ident;

    unimplemented!();
}

fn add_pos_field(mut data: Data) -> Data {
    if let Data::Struct(ref data_struct) = data {
        if let Fields::Named(ref fields) = data_struct.fields {
            fields.named.push()
        }
        else {
            panic!("Unable to add position to struct without named fields");
        }
    }
    else {
        panic!("Can only implement positionable for structs");
    }

    data
}

fn impl_positionable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl Positionable for #name {
            fn pos(&self) -> Vec2<u8> { self.pos }
            fn set_pos(&mut self, pos: Vec2<u8>) { self.pos = pos }
        }
    };

    gen.into()
}
