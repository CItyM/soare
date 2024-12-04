use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

#[proc_macro_derive(StructOfArrays)]
pub fn struct_of_arrays_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let soa_name = syn::Ident::new(&format!("{}SoA", name), name.span());

    let (field_idents, field_types): (Vec<_>, Vec<_>) = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| (field.ident.clone().unwrap(), field.ty.clone()))
                .unzip(),
            _ => {
                return Error::new_spanned(
                    input,
                    "StructOfArrays can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return Error::new_spanned(input, "StructOfArrays can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        pub struct #soa_name {
            len: usize,
            #(
                #field_idents: Vec<#field_types>,
            )*
        }

        impl #soa_name {
            pub fn from_instances(instances: Vec<#name>) -> Self {
                let len = instances.len();
                let mut soa = #soa_name {
                    len,
                    #(#field_idents: Vec::with_capacity(len)),*
                };

                for instance in instances {
                    #(soa.#field_idents.push(instance.#field_idents);)*
                }

                soa
            }

            pub fn len(&self) -> usize {
                self.len
            }

            pub fn is_empty(&self) -> bool {
                self.len == 0
            }

            pub fn push(&mut self, value: #name) {
                #(self.#field_idents.push(value.#field_idents);)*

                self.len += 1;
            }

            #(
                pub fn #field_idents(&self) -> &[#field_types] {
                    &self.#field_idents
                }
            )*

            pub fn pop(&mut self) -> Option<#name> {
                if self.len == 0 {
                    return None;
                }

                self.len -= 1;

                Some(#name {
                    #(#field_idents: self.#field_idents.pop().unwrap()),*
                })
            }

            pub fn remove(&mut self, index: usize) -> #name {
                self.len -= 1;

                #name {
                    #(#field_idents: self.#field_idents.remove(index)),*
                }
            }

            pub fn swap_remove(&mut self, index: usize) -> #name {
                self.len -= 1;

                #name {
                    #(#field_idents: self.#field_idents.swap_remove(index)),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
