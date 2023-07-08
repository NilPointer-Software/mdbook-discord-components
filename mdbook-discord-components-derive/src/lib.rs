use syn::{spanned::Spanned, DeriveInput, Expr, Lit, ExprLit, Data, DataStruct, Type, Path, Ident, PathArguments, AngleBracketedGenericArguments, GenericArgument};
use quote::{quote, quote_spanned, ToTokens};
use proc_macro2::{TokenStream, Span};
use convert_case::{Case, Casing};

// To anyone who knows how to write proc macros, I'm sorry

#[proc_macro_derive(Generatable, attributes(gen))]
pub fn derive_generatable(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as DeriveInput);

    let mut slot: Option<String> = None;
    for attr in input.attrs {
        if let Ok(expr) = attr.parse_args::<Expr>() {
            if let Expr::Assign(assign) = expr {
                let value = if let Expr::Lit(ExprLit { lit: Lit::Str(right), .. }) = *assign.right {
                    right.value()
                } else {
                    String::new()
                };
                let left_str = assign.left.to_token_stream().to_string();
                if &left_str == "slot" {
                    slot = Some(value);
                    continue;
                }
            }
        }
    }

    let hash_map_declare = if let Some(slot) = slot {
        quote! { let mut attr = std::collections::HashMap::<String, String>::from([("slot".to_owned(), #slot.to_owned())]); }
    } else {
        quote! { let mut attr = std::collections::HashMap::<String, String>::new(); }
    };

    let Data::Struct(DataStruct { fields, .. }) = input.data else {
        panic!("Generatable derive on a invalid data structure")
    };

    let attrs = fields.iter().map(|f| {
        let Type::Path(field_type) = &f.ty else {
            panic!("invalid field type variant '{}'", f.to_token_stream())
        };
        let field_name = f.ident.as_ref().unwrap();
        if let Some(last_segment) = field_type.path.segments.last() {
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = &last_segment.arguments {
                if let Some(GenericArgument::Type(Type::Path(inner_type))) = args.first() {
                    let attr_line = gen_attr_line(f.span(), field_name, &inner_type.path, false);
                    return quote! {
                        if let Some(#field_name) = self.#field_name {
                            #attr_line
                        }
                    }
                }
            }
        }
        gen_attr_line(f.span(), field_name, &field_type.path, true)
    });

    let name = input.ident;
    let tag_name = format!("discord-{}", name).to_case(Case::Kebab);
    quote! {
        impl crate::generators::Generatable for #name {
            fn name(&self) -> &str {
                #tag_name
            }

            fn attrubutes(self: Box<Self>) -> std::collections::HashMap<String, String> {
                #hash_map_declare
                #(#attrs)*
                attr
            }
        }
    }.into()
}

fn gen_attr_line(span: Span, field_name: &Ident, path: &Path, append_self: bool) -> TokenStream {
    let self_dot = if append_self {
        quote!{ self. }
    } else {
        quote!{}
    };
    let field_name_kebab = field_name.to_string().to_case(Case::Kebab);
    if path.is_ident("String") {
        quote_spanned! { span =>
            attr.insert(#field_name_kebab.to_owned(), #self_dot #field_name);
        }.into()
    } else if path.is_ident("bool") {
        quote_spanned! { span =>
            if #self_dot #field_name {
                attr.insert(#field_name_kebab.to_owned(), "".to_owned());
            }
        }.into()
    } else {
        quote_spanned! { span =>
            attr.insert(#field_name_kebab.to_owned(), #self_dot #field_name.to_string());
        }.into()
    }
}
