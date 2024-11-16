extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Fields, FieldsNamed, PathArguments, PathSegment, Type};

#[proc_macro_derive(ModelMerge)]
pub fn model_merge_derive(input: TokenStream) -> TokenStream {
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_model_merge(&ast)
}

fn impl_model_merge(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let Data::Struct(data_struct) = &ast.data else {
        panic!("only support struct");
    };

    let Fields::Named(fields) = &data_struct.fields else {
        panic!("only support named.");
    };

    let result = fields.named.iter().map(|field| {
        let ty = &field.ty;
        let ident = &field.ident;

        if let Type::Path(typ_path) = ty {
            let options: Vec<&PathSegment> = typ_path.path.segments.iter().filter(|s| s.ident == "Option").collect();
            
            if let Some(last) = options.last() {

                let PathArguments::AngleBracketed(arg) = &last.arguments else {
                    panic!("not found arg");
                };

                let syn::GenericArgument::Type(inner_type) = arg.args.first().unwrap().into_value() else {
                    panic!("not inner type arg");
                };

                match inner_type {
                    Type::Path(_) => {
                        quote! {
                            if let Some(ref mut self_value) = self.#ident {
                                if let Some(ref other_value) = other.#ident {
                                    self_value.merge(other_value);
                                }
                            } else {
                                self.#ident = other.#ident.clone();
                            }
                        }
                    },
                    _ => {
                        quote! {
                            // self.#ident = other.#ident.clone();
                        }
                    }
                }

            } else {
                quote! {
                    // self.#ident = other.#ident.clone();
                }
            }

        } else {
            quote! {
                // self.#ident = other.#ident.clone();
            }
        }
    });

    let gen = quote! {
        impl ModelMerge for #name {
            fn merge(&mut self, other: &Self) {
                #( #result ) *
            }
        }
    };

    gen.into()
}