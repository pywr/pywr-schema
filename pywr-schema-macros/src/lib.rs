use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(PywrNode)]
pub fn pywr_node_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_pywr_node_derive(&input)
}

fn impl_pywr_node_derive(ast: &syn::DeriveInput) -> TokenStream {
    // Name of the node type
    let name = &ast.ident;

    if let syn::Data::Struct(data) = &ast.data {
        struct ParamField {
            ident: syn::Ident,
            optional: bool,
        }

        let parameter_fields: Vec<ParamField> = data
            .fields
            .iter()
            .filter_map(|field| {
                let field_ident = field.ident.as_ref()?;

                match &field.ty {
                    syn::Type::Path(type_path) if type_path.qself.is_none() => {
                        match type_path.path.segments.last() {
                            Some(last_segment) => {
                                let ident = &last_segment.ident;
                                if ident == "Option" {
                                    let first_arg = match &last_segment.arguments {
                                        syn::PathArguments::AngleBracketed(params) => {
                                            params.args.first()
                                        }
                                        _ => None,
                                    };

                                    let arg_ty = match first_arg {
                                        Some(syn::GenericArgument::Type(ty)) => Some(ty),
                                        _ => None,
                                    };

                                    let arg_type_path = match arg_ty {
                                        Some(ty) => match ty {
                                            syn::Type::Path(type_path)
                                                if type_path.qself.is_none() =>
                                            {
                                                Some(type_path)
                                            }
                                            _ => None,
                                        },
                                        None => None,
                                    };

                                    let last_segment = match arg_type_path {
                                        Some(type_path) => type_path.path.segments.last(),
                                        None => None,
                                    };

                                    match last_segment {
                                        Some(last_segment) => {
                                            let ident = &last_segment.ident;
                                            if (ident == "ParameterValue")
                                                | (ident == "ParameterValues")
                                            {
                                                Some(ParamField {
                                                    ident: field_ident.clone(),
                                                    optional: true,
                                                })
                                            } else {
                                                None
                                            }
                                        }
                                        None => None,
                                    }
                                } else if (ident == "ParameterValue") | (ident == "ParameterValues")
                                {
                                    let optional = match type_path.path.segments.first() {
                                        Some(first_segment) => first_segment.ident == "Option",
                                        None => false,
                                    };

                                    Some(ParamField {
                                        ident: field_ident.clone(),
                                        optional,
                                    })
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    }
                    _ => None,
                }
            })
            .collect();

        // ::parameters() method
        let inserts = parameter_fields
            .iter()
            .map(|param_field| {
                let ident = &param_field.ident;
                let key = ident.to_string();
                if param_field.optional {
                    quote! {
                        if let Some(p) = &self.#ident {
                            attributes.insert(#key, p.into());
                        }
                    }
                } else {
                    quote! {
                        let #ident = &self.#ident;
                        attributes.insert(#key, #ident.into());
                    }
                }
            })
            .collect::<Vec<_>>();

        let inserts_mut = parameter_fields
            .iter()
            .map(|param_field| {
                let ident = &param_field.ident;
                let key = ident.to_string();
                if param_field.optional {
                    quote! {
                        if let Some(p) = &mut self.#ident {
                            attributes.insert(#key, p.into());
                        }
                    }
                } else {
                    quote! {
                        let #ident = &mut self.#ident;
                        attributes.insert(#key, #ident.into());
                    }
                }
            })
            .collect::<Vec<_>>();

        let expanded = quote! {
            impl #name {
                pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
                    let mut attributes = HashMap::new();
                    #(
                        #inserts
                    )*
                    attributes
                }

                pub fn parameters_mut(&mut self) -> HashMap<&str, ParameterValueTypeMut> {
                    let mut attributes = HashMap::new();
                    #(
                        #inserts_mut
                    )*
                    attributes
                }
            }
        };

        // Hand the output tokens back to the compiler.
        TokenStream::from(expanded)
    } else {
        panic!("Only structs are supported for #[derive(PywrNode)]")
    }
}
