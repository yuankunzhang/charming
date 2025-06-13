#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
CharmingSetters is a derive macro for the Charming chart rendering library. It is used to remove a lot of boilerplate code in the library by implementing common methods to set the fields of structs.

Example without the macro
```rust
use serde::{Serialize, Deserialize};
use charming::component::Title;
use charming::element::{Color, Tooltip};
use charming::datatype::{DataFrame, DataPoint};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Component {
    title: Vec<Title>,
    tooltip: Option<Tooltip>,
    color: Vec<Color>,
    position: Option<(String, String)>,
    data: DataFrame

    // many fields cut for brevity
}
// For setting these fields with the setter pattern we would need to implement the following
// methods manually when we are not using the derive macro
impl Component {
    pub fn new() -> Self {
        Self {
            title: Vec::new(),
            tooltip: None,
            color: Vec::new(),
            position: None,
            data: DataFrame::default(),
        }
    }
    pub fn title<T: Into<Title>>(mut self, title: T) -> Self {
        self.title.push(title.into());
        self
    }
    pub fn tooltip<T: Into<Tooltip>>(mut self, tooltip: T) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
    pub fn color<C: Into<Color>>(mut self, color: Vec<C>) -> Self {
        self.color = color.into_iter().map(|c| c.into()).collect();
        self
    }
    pub fn position<S: Into<String>>(mut self, position: (S, S)) -> Self {
        self.position = Some((position.0.into(), position.1.into()));
        self
    }
    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
    // many methods cut for brevity
}
```

Example with the macro
```rust
use serde::{Serialize, Deserialize};
use charming::component::Title;
use charming::element::{Color, Tooltip};
use charming::datatype::{DataFrame, DataPoint};
use charming_macros::CharmingSetters;

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
struct Component {
    title: Vec<Title>,
    tooltip: Option<Tooltip>,
    #[charming_set_vec]
    color: Vec<Color>,
    #[charming_skip_setter]
    position: Option<(String, String)>,
    data: DataFrame

    // many fields cut for brevity
}

impl Component {
    pub fn position<S: Into<String>>(mut self, position: (S, S)) -> Self {
        self.position = Some((position.0.into(), position.1.into()));
        self
    }
}
// All other methods from the previous example are now getting generated automatically

```
*/
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_derive(
    CharmingSetters,
    attributes(charming_skip_setter, charming_type, charming_set_vec)
)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let struct_ident = input.ident;
    let mut fields_init_values = Vec::with_capacity(input.fields.len());
    let mut fields_setter = Vec::with_capacity(input.fields.len());

    for field in input.fields {
        let mut generate_setter = true;

        let field_ident = field.ident.unwrap();
        let field_string_shorthand = field_ident
            .to_string()
            .chars()
            .next()
            .unwrap()
            .to_string()
            .to_uppercase();

        let field_ident_shorthand = Ident::new(&field_string_shorthand, Span::call_site());

        match field.ty {
            syn::Type::Path(type_path) => {
                // type_wrapper will be the first type `type_path: Option<T> -> type_wrapper = "Option"`
                let type_wrapper = &type_path.path.segments.first().unwrap().ident.to_string();

                for attribute in field.attrs {
                    match attribute.meta {
                        syn::Meta::Path(path) => {
                            if let Some(segment) = path.segments.last() {
                                match segment.ident.to_string().as_str() {
                                    // `charming_skip_setter` will skip the automatic setter implementation
                                    "charming_skip_setter" => {
                                        if type_wrapper == "Option" {
                                            fields_init_values.push(quote! { #field_ident: None });
                                        } else if type_wrapper == "Vec" {
                                            fields_init_values
                                                .push(quote! { #field_ident: Vec::new() });
                                        } else {
                                            fields_init_values
                                                .push(quote! { #field_ident: Default::default() });
                                        }

                                        generate_setter = false;
                                    }
                                    // `charming_set_vec` will implement a setter method where the vec is set at once
                                    "charming_set_vec" => {
                                        let field_string_lowercase_shorthand =
                                            field_string_shorthand.to_lowercase();
                                        let field_ident_lowercase_shorthand = Ident::new(
                                            &field_string_lowercase_shorthand,
                                            Span::call_site(),
                                        );
                                        let field_generic_type = &type_path
                                            .path
                                            .segments
                                            .iter()
                                            .next()
                                            .unwrap()
                                            .arguments;

                                        fields_init_values
                                            .push(quote! { #field_ident: Vec::new() });

                                        // This implements a method that looks like this for a field `dimension: Vec<Dimension>`
                                        //```rust
                                        //pub fn dimensions<D: Into<Dimension>>(mut self, dimensions: Vec<D>) -> Self {
                                        //    self.dimensions = dimensions.into_iter().map(|d| d.into()).collect();
                                        //    self
                                        //}
                                        // ```
                                        fields_setter.push(quote! {
                                            pub fn #field_ident<#field_ident_shorthand: Into #field_generic_type>(mut self, #field_ident: Vec<#field_ident_shorthand>) -> Self {
                                                self.#field_ident = #field_ident.into_iter().map(|#field_ident_lowercase_shorthand| #field_ident_lowercase_shorthand.into()).collect();
                                                self
                                            }
                                        });

                                        generate_setter = false;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        syn::Meta::List(_meta_list) => (),
                        syn::Meta::NameValue(meta_name_value) => {
                            if let Some(segment) = meta_name_value.path.segments.last() {
                                if let "charming_type" = segment.ident.to_string().as_str() {
                                    match &meta_name_value.value {
                                        syn::Expr::Lit(expr_lit) => {
                                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                                let value = lit_str.value();

                                                // This sets the String to whatever value was provided in the `new` method, it is used to provide a value to type_
                                                fields_init_values.push(
                                                    quote! { #field_ident: #value.to_string() },
                                                )
                                            }
                                        }
                                        _ => {
                                            panic!("charming_type needs a string literal")
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if type_wrapper == "Option" && generate_setter {
                    let field_generic_type =
                        &type_path.path.segments.iter().next().unwrap().arguments;

                    fields_init_values.push(quote! { #field_ident: None });

                    // This implements a method that looks like this for a field `id: Option<String>`
                    //```rust
                    //pub fn id<I: Into<String>>(mut self, id: I) -> Self {
                    //    self.id = Some(id.into());
                    //    self
                    //}
                    // ```
                    fields_setter.push(quote! {
                        pub fn #field_ident<#field_ident_shorthand: Into #field_generic_type >(mut self, #field_ident: #field_ident_shorthand) -> Self {self.#field_ident = Some(#field_ident.into());
                          self
                        }
                    });
                } else if type_wrapper == "Vec" && generate_setter {
                    let field_generic_type =
                        &type_path.path.segments.iter().next().unwrap().arguments;

                    fields_init_values.push(quote! { #field_ident: Vec::new() });

                    // This implements a method that looks like this for a field `series: Vec<Series>`
                    //```rust
                    //pub fn series<S: Into<Series>>(mut self, series: S) -> Self {
                    //    self.series.push(series.into());
                    //    self
                    //}
                    // ```
                    fields_setter.push(quote! {
                        pub fn #field_ident<#field_ident_shorthand: Into #field_generic_type >(mut self, #field_ident: #field_ident_shorthand) -> Self {self.#field_ident.push(#field_ident.into());
                        self}
                    });
                } else if type_wrapper == "DataFrame" && generate_setter {
                    fields_init_values.push(quote! { #field_ident: DataFrame::default() });

                    fields_setter.push(quote! {
                        pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
                            self.data = data.into_iter().map(|d| d.into()).collect();
                            self
                        }
                    });
                };
            }
            _ => todo!(),
        }
    }

    quote! {
        impl #struct_ident {
            pub fn new() -> #struct_ident {
                #struct_ident {
                    #(#fields_init_values),*
                }
            }

            #(#fields_setter)*
        }

        impl Default for #struct_ident {
            fn default() -> Self {
                Self::new()
            }
        }
    }
    .into()
}
