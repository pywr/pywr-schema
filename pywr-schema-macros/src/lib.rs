use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(PywrNode)]
pub fn pywr_node_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_parameter_references_derive(&input)
}

#[proc_macro_derive(PywrParameter)]
pub fn pywr_parameter_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let mut expanded = impl_parameter_references_derive(&input);
    expanded.extend(impl_parameter_resource_paths_derive(&input));

    expanded
}

/// Generates a [`TokenStream`] containing the implementation of two methods, `parameters`
/// and `parameters_mut`, for the given struct.
///
/// The `parameters` method returns a [`HashMap`] of parameter names to [`ParameterValueType`],
/// and the `parameters_mut` method returns a [`HashMap`] of parameter names to [`ParameterValueTypeMut`].
/// This is intended to be used for nodes and parameter structs in the Pywr schema.
///
/// Currently the implementation is limited to simple type definitions such as `Option<ParameterValue>` or `ParameterValue`.
fn impl_parameter_references_derive(ast: &syn::DeriveInput) -> TokenStream {
    // Name of the node type
    let name = &ast.ident;

    if let syn::Data::Struct(data) = &ast.data {
        struct ParamField {
            field_name: syn::Ident,
            optional: bool,
        }

        let parameter_fields: Vec<ParamField> = data
            .fields
            .iter()
            .filter_map(|field| {
                let field_ident = field.ident.as_ref()?;

                match type_to_ident(&field.ty) {
                    Some(PywrField::Optional(ident)) => {
                        is_parameter_ident(&ident).then_some(ParamField {
                            field_name: field_ident.clone(),
                            optional: true,
                        })
                    }
                    Some(PywrField::Required(ident)) => {
                        is_parameter_ident(&ident).then_some(ParamField {
                            field_name: field_ident.clone(),
                            optional: false,
                        })
                    }
                    None => None,
                }
            })
            .collect();

        // Insert statements for non-mutable version
        let inserts = parameter_fields
            .iter()
            .map(|param_field| {
                let ident = &param_field.field_name;
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

        // Insert statements for non-mutable version
        let inserts_mut = parameter_fields
            .iter()
            .map(|param_field| {
                let ident = &param_field.field_name;
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

        // Create the two parameter methods using the insert statements
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
        panic!("Only structs are supported for #[derive(PywrNode)] or #[derive(PywrParameter)]")
    }
}

/// Generates a [`TokenStream`] containing the implementation `resource_paths`.
fn impl_parameter_resource_paths_derive(ast: &syn::DeriveInput) -> TokenStream {
    // Name of the node type
    let name = &ast.ident;

    if let syn::Data::Struct(data) = &ast.data {
        struct PathField {
            field_name: syn::Ident,
            ty: PathFieldType,
            optional: bool,
        }

        let path_fields: Vec<PathField> = data
            .fields
            .iter()
            .filter_map(|field| {
                let field_ident = field.ident.as_ref()?;

                match type_to_ident(&field.ty) {
                    Some(PywrField::Optional(ident)) => {
                        ident_to_path_type(&ident).map(|field_type| PathField {
                            field_name: field_ident.clone(),
                            ty: field_type,
                            optional: true,
                        })
                    }
                    Some(PywrField::Required(ident)) => {
                        ident_to_path_type(&ident).map(|field_type| PathField {
                            field_name: field_ident.clone(),
                            ty: field_type,
                            optional: false,
                        })
                    }
                    None => None,
                }
            })
            .collect();

        // Insert statements for non-mutable version
        let inserts = path_fields
            .iter()
            .map(|param_field| {
                let ident = &param_field.field_name;

                match &param_field.ty {
                    PathFieldType::ExternalDataRef => {
                        if param_field.optional {
                            quote! {
                                if let Some(external) = &self.#ident {
                                    resource_paths.push(external.url.clone());
                                }
                            }
                        } else {
                            quote! {
                                resource_paths.push(self.#ident.url.clone());
                            }
                        }
                    }
                    PathFieldType::PathBuf => {
                        if param_field.optional {
                            quote! {
                                if let Some(p) = &self.#ident {
                                    resource_paths.push(p.clone());
                                }
                            }
                        } else {
                            quote! {
                                resource_paths.push(self.#ident.clone());
                            }
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        let updates = path_fields
            .iter()
            .map(|param_field| {
                let ident = &param_field.field_name;

                match &param_field.ty {
                    PathFieldType::ExternalDataRef => {
                        if param_field.optional {
                            quote! {
                                if let Some(external) = &mut self.#ident {
                                    if let Some(new_path) = new_paths.get(&external.url) {
                                        external.url = new_path.clone();
                                    }
                                }
                            }
                        } else {
                            quote! {
                                if let Some(new_path) = new_paths.get(&self.#ident.url) {
                                    self.#ident.url = new_path.clone();
                                }
                            }
                        }
                    }
                    PathFieldType::PathBuf => {
                        if param_field.optional {
                            quote! {
                                if let Some(path) = &mut self.#ident {
                                    if let Some(new_path) = new_paths.get(path) {
                                        *path = new_path.clone();
                                    }
                                }
                            }
                        } else {
                            quote! {
                                if let Some(new_path) = new_paths.get(&self.#ident) {
                                    self.#ident = new_path.clone();
                                }
                            }
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        // Create the two parameter methods using the insert statements
        let expanded = quote! {
            impl #name {
                pub fn resource_paths(&self) -> Vec<PathBuf> {
                    let mut resource_paths = Vec::new();
                    #(
                        #inserts
                    )*
                    resource_paths
                }

                pub fn update_resource_paths(&mut self, new_paths: &HashMap<PathBuf, PathBuf>) {
                    #(
                        #updates
                    )*
                }
            }
        };

        // Hand the output tokens back to the compiler.
        TokenStream::from(expanded)
    } else {
        panic!("Only structs are supported for #[derive(PywrNode)] or #[derive(PywrParameter)]")
    }
}

enum PywrField {
    Optional(syn::Ident),
    Required(syn::Ident),
}

/// Returns the last segment of a type path as an identifier
fn type_to_ident(ty: &syn::Type) -> Option<PywrField> {
    match ty {
        syn::Type::Path(type_path) if type_path.qself.is_none() => {
            match type_path.path.segments.last() {
                Some(last_segment) => {
                    let ident = &last_segment.ident;

                    if ident == "Option" {
                        let first_arg = match &last_segment.arguments {
                            syn::PathArguments::AngleBracketed(params) => params.args.first(),
                            _ => None,
                        };

                        let arg_ty = match first_arg {
                            Some(syn::GenericArgument::Type(ty)) => Some(ty),
                            _ => None,
                        };

                        let arg_type_path = match arg_ty {
                            Some(ty) => match ty {
                                syn::Type::Path(type_path) if type_path.qself.is_none() => {
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
                                Some(PywrField::Optional(ident.clone()))
                            }
                            None => None,
                        }
                    } else {
                        Some(PywrField::Required(ident.clone()))
                    }
                }
                None => None,
            }
        }
        _ => None,
    }
}

fn is_parameter_ident(ident: &syn::Ident) -> bool {
    (ident == "ParameterValue") || (ident == "ParameterValues")
}

enum PathFieldType {
    ExternalDataRef,
    PathBuf,
}

fn ident_to_path_type(ident: &syn::Ident) -> Option<PathFieldType> {
    if ident == "ExternalDataRef" {
        Some(PathFieldType::ExternalDataRef)
    } else if ident == "PathBuf" {
        Some(PathFieldType::PathBuf)
    } else {
        None
    }
}
