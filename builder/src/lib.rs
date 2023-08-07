use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, DeriveInput, Expr, GenericArgument, Ident,
    MetaNameValue, Token, Type,
};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let command_ident = input.ident;
    let builder_ident = format_ident!("{}Builder", command_ident);

    let syn::Data::Struct(struct_data) = input.data else { panic!("Only structs are supported") };
    for name in ["executable", "args", "env", "current_dir"] {
        if !struct_data
            .fields
            .iter()
            .any(|f| f.ident.clone().unwrap().to_string() == name)
        {
            panic!("Missing field {}", name);
        }
    }

    let mut setters = quote! {};
    let mut build_func_body = quote! {};
    let mut builder_struct_body = quote! {};
    let mut builder_init_body = quote! {};

    struct_data.fields.iter().for_each(|f| {
        let field_name = f.ident.clone().unwrap();
        let field_type = &f.ty;
        let option_body = type_arrow_body(field_type, "Option");
        let vec_body = type_arrow_body(field_type, "Vec");
        let vec_func_name = f.attrs.iter().find_map(|a| {
            if !a.path().is_ident("builder") { return None; };
            
            let name_values: Punctuated<MetaNameValue, Token![,]> = a.parse_args_with(Punctuated::parse_terminated).unwrap();
            for nv in name_values {
                if nv.path.is_ident("each") {
                    let Expr::Lit(lit) = nv.value else {
                        panic!("Failed to parse attribute: attribute value is not a literal");
                    };
                    let syn::Lit::Str(lit) = lit.lit else {
                        panic!("Failed to parse attribute: attribute value is not a string literal");
                    };
                    return Some(lit.value());
                };
            };
            None
        });
        let for_each_vec_func = vec_func_name.is_some();
        let vec_func_name = Ident::new(&vec_func_name.unwrap_or(field_name.to_string()), field_name.span());

        match &option_body {
            Some(field_optional_type) => {
                setters = quote! {
                    #setters
                    pub fn #vec_func_name(&mut self, value: #field_optional_type) -> &mut Self {
                        self.#field_name = Some(value);
                        self
                    }
                };
                builder_struct_body = quote! {
                    #builder_struct_body
                    #field_name: #field_type,
                };
                builder_init_body = quote! {
                    #builder_init_body
                    #field_name: None,
                };
                build_func_body = quote! {
                    #build_func_body
                    #field_name: self.#field_name.clone(),
                };
            }
            None => match &vec_body {
                Some(field_vec_type) => {
                    if for_each_vec_func {
                        setters = quote! {
                            #setters
                            pub fn #vec_func_name(&mut self, element: #field_vec_type) -> &mut Self {
                                self.#field_name.push(element);
                                self
                            }
                        };
                    } else {                        
                        setters = quote! {
                            #setters
                            pub fn #vec_func_name(&mut self, element: #field_type) -> &mut Self {
                                self.#field_name = element;
                                self
                            }
                        };
                    }
                    builder_struct_body = quote! {
                        #builder_struct_body
                        #field_name: #field_type,
                    };
                    builder_init_body = quote! {
                        #builder_init_body
                        #field_name: Vec::new(),
                    };
                    build_func_body = quote! {
                        #build_func_body
                        #field_name: self.#field_name.clone(),
                    };
                }
                None => {
                    setters = quote! {
                        #setters
                        pub fn #vec_func_name(&mut self, value: #field_type) -> &mut Self {
                            self.#field_name = Some(value);
                            self
                        }
                    };
                    builder_struct_body = quote! {
                        #builder_struct_body
                        #field_name: Option<#field_type>,
                    };
                    builder_init_body = quote! {
                        #builder_init_body
                        #field_name: None,
                    };
                    build_func_body = quote! {
                        #build_func_body
                        #field_name: self.#field_name.clone().ok_or(concat!(stringify!(#field_name), " is required"))?,
                    };
                }
            },
        };
    });

    let expanded = quote! {
        impl #command_ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #builder_init_body
                }
            }
        }

        pub struct #builder_ident {
            #builder_struct_body
        }

        impl #builder_ident {
            #setters

            pub fn build(&mut self) -> Result<#command_ident, Box<dyn std::error::Error>> {
                Ok(#command_ident {
                    #build_func_body
                })
            }
        }
    };

    TokenStream::from(expanded)
}

fn type_arrow_body(field_type: &Type, name: &str) -> Option<Type> {
    let Type::Path(path) = field_type else { return None; };
    let path = &path.path;

    if !(path.segments.len() == 1 && path.segments.iter().next().unwrap().ident == name) {
        return None;
    };

    let option_path = &path.segments.iter().next().unwrap().arguments;
    let syn::PathArguments::AngleBracketed(option_path) = option_path else { return None; };

    let Some(option_path) = option_path.args.first() else { return None; };
    let GenericArgument::Type(option_type) = option_path else { return None; };

    Some(option_type.clone())
}