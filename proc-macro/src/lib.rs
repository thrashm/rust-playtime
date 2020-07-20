extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput };

#[proc_macro_derive(XmlSerialize)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed {ref named, ..}), .. }) = ast.data {
        named
    } else {
        unimplemented!();
    };
    let field_elements = fields.iter().map(|field| {
        let wrapped_field_name = field.clone().ident;
        wrapped_field_name.unwrap()
    });
    let mut xml_format = String::new();

    xml_format.push_str(&format!("<{0}>", format!("{}", name)));

    let mut i = 0;
    for field_element in field_elements {
        xml_format.push_str(&format!("<{0}>{{{1}}}</{0}>",  field_element, i));
        i = i + 1;
    }

    xml_format.push_str(&format!("</{0}>", format!("{}", name)));

    let assignments = fields.iter().map(|field| {
        let wrapped_field_name = field.clone().ident;
        let fc = wrapped_field_name.unwrap();
        quote! {
            ,self.#fc
        }
    });

    let expanded = quote! {

        impl stuff::XSerialize for #name {
            fn xml_serialize(&self) -> String {
                String::from(format!(#xml_format
                    #(#assignments)*
                ))
            }
        }
    };

    expanded.into()
}