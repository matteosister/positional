use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, Fields, FieldsNamed};

use crate::attributes_parsing::{create_row_attributes, FieldAlignment, parse_meta};
use crate::type_parsing::extract_option_type;

pub fn from_positional_for_struct(ast: syn::DeriveInput) -> TokenStream {
    let type_name = ast.ident;
    let type_span = type_name.span();

    match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let from_positional = match create_from_positional(&fields) {
                    Ok(from_positional_stream) => from_positional_stream,
                    Err(error) => return error,
                };
                quote! {
                    impl FromPositionalRow for #type_name {
                        #from_positional
                    }
                }
            }
            Fields::Unnamed(_) => {
                quote_spanned!(type_span=> compile_error!("only structs with named fields! This is an unnamed struct"))
            }
            Fields::Unit => {
                quote_spanned!(type_span=> compile_error!("only structs with named fields! This is a unit struct"))
            }
        },
        Data::Enum(_) => {
            quote_spanned!(type_span=> compile_error!("only structs! This is an enum"))
        }
        Data::Union(_) => {
            quote_spanned!(type_span=> compile_error!("only structs! This is a union type"))
        }
    }
}

fn create_from_positional(
    fields: &FieldsNamed,
) -> Result<TokenStream, TokenStream> {
    let fields: Vec<TokenStream> = parse_fields_into_struct_builder_stream(fields)?;
    Ok(quote! {
        fn parse(row: impl ToString) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
            let row_string = row.to_string();
            Ok(Self {
                #(#fields),*
            })
        }
    })
}

fn parse_fields_into_struct_builder_stream(
    fields: &FieldsNamed,
) -> Result<Vec<TokenStream>, TokenStream> {
    let mut field_token_streams = vec![];
    let mut offset = 0;
    for field in &fields.named {
        // it's ok to unwrap here, because we are in a Struct with named fields
        let field_ident = field.ident.as_ref().unwrap();
        for attr in &field.attrs {
            if attr.path.is_ident(super::FIELD_ATTR_NAME) {
                let meta = attr.parse_meta().expect("unable to parse meta");
                let mut attrs = vec![];
                let field_type = extract_option_type(&field.ty);
                parse_meta(&mut attrs, meta);
                let row_attributes = create_row_attributes(attrs)?;
                let size = row_attributes.size;
                let filler = row_attributes.filler;
                let align = row_attributes.align == FieldAlignment::Left;
                let output = match field_type {
                    // simple type definition like i32 or String
                    None => {
                        quote! {
                            #field_ident: PositionalParsedField::new(row.to_string(), #offset, #size, #filler, #align).to_value().parse()?
                        }
                    }
                    // optional type definition like Option<i32> or Option<String>
                    Some(_) => {
                        quote! {
                            #field_ident: PositionalParsedField::new(row.to_string(), #offset, #size, #filler, #align).to_value().parse().ok()
                        }
                    }
                };
                offset += size;
                field_token_streams.push(output);
            }
        }
    }
    Ok(field_token_streams)
}
