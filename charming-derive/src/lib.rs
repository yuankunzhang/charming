#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
CharmingSetter is a derive macro for the Charming chart rendering library. It is used to remove a lot of boilerplate code
in the library by implementing common methods to set the fields of structs.

Example without the macro
```rust
use serde::Serialize;
use charming::component::Title;
use charming::element::Tooltip;

#[derive(Serialize, Debug, Clone)]
struct Chart {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    title: Vec<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip: Option<Tooltip>,

    // many fields cut for brevity
}

// For setting these fields with the setter pattern we would need to implement the following
// methods manually when we are not using the derive macro
impl Chart {
    pub fn new() -> Self {
        Self {
            title: Vec::new(),
            tooltip: None,
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

    // many methods cut for brevity
}

// Example chart creation
let chart = Chart::new().title(Title::new().text("Title")).tooltip(Tooltip::new());
```

Example with the macro
```rust
use serde::Serialize;
use charming::component::Title;
use charming::element::Tooltip;
use charming_derive::CharmingSetter;

#[derive(Serialize, Debug, Clone, CharmingSetter)]
struct Chart {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    title: Vec<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip: Option<Tooltip>,

    // many fields cut for brevity
}
// The setter methods from the example above now get implemented automatically by CharmingSetter

// Example chart creation
let chart = Chart::new().title(Title::new().text("Title")).tooltip(Tooltip::new());
```

Field attributes charming_type and charming_skip_setter
```rust
use serde::Serialize;
use charming::datatype::DataPoint;
use charming_derive::CharmingSetter;

#[derive(Serialize, Debug, Clone, CharmingSetter)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    // charming_type gets used here to provide the value "line" as the default value for
    // the field 'type_' when calling Line::new() and also removes the method to set the field
    #[serde(rename = "type")]
    #[charming_type = "line"]
    type_: String,

    // cut for brevity

    // charming_skip_setter gets used here to remove the default implementation of the setter
    // method for this field, a manual method to set this field needs to be provided instead
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[charming_skip_setter]
    data: Vec<DataPoint>,
}

impl Line {
    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}

```
*/

use syn::visit::Visit;

#[proc_macro_derive(CharmingSetter, attributes(charming_skip_setter, charming_type))]
pub fn charming_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ident = input.ident;
    let mut field_list = vec![];
    let mut method_list = vec![];

    let fields = match &input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => fields.named.iter().collect::<Vec<_>>(),
            _ => unimplemented!(
                "Can't implement CharmingSetter on a unit struct or a struct with unnamed fields"
            ),
        },
        syn::Data::Enum(_data_enum) => unimplemented!("Can't implement CharmingSetter on an Enum"),
        syn::Data::Union(_data_union) => {
            unimplemented!("Can't implement CharmingSetter on a Union")
        }
    };

    'outer: for field in fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_types = gather_all_type_reprs(&field.ty);
        let type_ident =
            syn::Ident::new(field_types.last().unwrap(), proc_macro2::Span::call_site());
        let shorthand_char = field_types.last().unwrap().chars().next().unwrap();
        let shorthand = syn::Ident::new(
            &shorthand_char.to_string().to_uppercase(),
            proc_macro2::Span::call_site(),
        );

        for attribute in &field.attrs {
            match &attribute.meta {
                syn::Meta::Path(path) => {
                    if let Some(segment) = path.segments.last() {
                        if let "charming_skip_setter" = segment.ident.to_string().as_str() {
                            if field_types[0].starts_with("Option") {
                                field_list.push(quote::quote! { #field_ident: None, });
                            } else if field_types[0].starts_with("Vec") {
                                field_list
                                    .push(quote::quote! { #field_ident: std::vec::Vec::new(), });
                            }
                            continue 'outer;
                        }
                    }
                }
                syn::Meta::List(_meta_list) => {}
                syn::Meta::NameValue(meta_name_value) => {
                    if let Some(segment) = meta_name_value.path.segments.last() {
                        if segment.ident.to_string().starts_with("charming_type") {
                            match &meta_name_value.value {
                                syn::Expr::Lit(expr_lit) => {
                                    if let syn::Lit::Str(string_lit) = &expr_lit.lit {
                                        let value = string_lit.value();
                                        field_list.push(
                                            quote::quote! { #field_ident: #value.to_string(), },
                                        );
                                    } else {
                                        unimplemented!("Expected string literal for charming_type")
                                    }
                                    continue 'outer;
                                }
                                _ => {
                                    unimplemented!("Can't implement non Literals for charming_type")
                                }
                            }
                        }
                    }
                }
            }
        }
        if field_types[0].starts_with("Option") {
            field_list.push(quote::quote! { #field_ident: None, });
            method_list.push(quote::quote! {
                pub fn #field_ident<#shorthand: Into<#type_ident>>(mut self, #field_ident: #shorthand) -> Self {
                    self.#field_ident = Some(#field_ident.into());
                    self
                }
            });
        } else if field_types[0].starts_with("Vec") {
            field_list.push(quote::quote! { #field_ident: std::vec::Vec::new(), });
            method_list.push(quote::quote! {
                pub fn #field_ident<#shorthand: Into<#type_ident>>(mut self, #field_ident: #shorthand) -> Self {
                    self.#field_ident.push(#field_ident.into());
                    self
                }
            });
        }
    }

    quote::quote!(
        impl #ident {
            pub fn new() -> Self {
                Self {
                    #(#field_list)*
                }
            }
            #(#method_list)*
        }
    )
    .into()
}

// Following code is from enso_macro_utils but supports syn2.0

#[derive(Default)]
struct TypeGatherer<'ast> {
    /// Observed types accumulator.
    pub types: Vec<&'ast syn::TypePath>,
}

impl<'ast> syn::visit::Visit<'ast> for TypeGatherer<'ast> {
    fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
        self.types.push(node);
        syn::visit::visit_type_path(self, node);
    }
}

fn gather_all_types(node: &syn::Type) -> Vec<&syn::TypePath> {
    let mut type_gatherer = TypeGatherer::default();
    type_gatherer.visit_type(node);
    type_gatherer.types
}

fn gather_all_type_reprs(node: &syn::Type) -> Vec<String> {
    gather_all_types(node).iter().map(repr).collect()
}

fn repr<T: quote::ToTokens>(t: &T) -> String {
    quote::quote!(#t).to_string()
}
