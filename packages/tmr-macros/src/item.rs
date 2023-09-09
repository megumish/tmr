use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, ItemImpl};
use tracing::trace;

#[tracing::instrument(level = "info")]
pub fn item(item_label: &str, impl_definition: &ItemImpl) -> TokenStream {
    let struct_ty = impl_definition.self_ty.as_ref();
    let println_result_ident = Ident::new("result", proc_macro::Span::call_site().into());
    let println_fn_steps = impl_definition
        .items
        .iter()
        .map(|item| match item {
            syn::ImplItem::Fn(method) => {
                let method_ident = &method.sig.ident;
                let format_str = format!("{}.{{}} = {{}}\n", item_label);
                quote!(
                    result.push_str(&format!(#format_str, stringify!(#method_ident), tmr::ToToml::to_toml(&self.#method_ident())));
                )
            }
            _ => panic!("Only methods are allowed in impl blocks"),
        })
        .collect::<Vec<_>>();
    trace!(
        "println_fn_steps:\n{:#?}",
        quote!(#(#println_fn_steps)*).to_string()
    );
    let method_definitions = impl_definition
        .items
        .iter()
        .map(|item| match item {
            syn::ImplItem::Fn(method) => method.clone(),
            _ => panic!("Only methods are allowed in impl blocks"),
        })
        .collect::<Vec<_>>();
    trace!(
        "method_definitions:\n{:#?}",
        quote!(#(#method_definitions)*).to_string()
    );
    let result = quote!(
        impl #struct_ty {
            fn println_fn(&self) -> String {
                let mut #println_result_ident = String::new();
                #(#println_fn_steps)*
                result
            }
            #(#method_definitions)*
        }
    );
    trace!("result:\n{:#?}", result.to_string());
    result.into()
}

#[tracing::instrument(level = "info")]
pub fn derive_item(item_label: &str, input: &DeriveInput) -> TokenStream {
    let struct_ident = &input.ident;
    let raw_fields = match &input.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("Package must be a struct"),
    };
    let field_assigns = raw_fields
        .iter()
        .map(|field| {
            let field_ident = field.ident.as_ref().unwrap();
            let field_value = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("value"))
                .unwrap()
                .parse_args::<syn::LitStr>()
                .unwrap();
            quote!(let #field_ident = #field_value.try_into().unwrap();)
        })
        .collect::<Vec<_>>();
    trace!(
        "field_assigns:\n{:#?}",
        quote!(#(#field_assigns)*).to_string()
    );
    let fields = raw_fields
        .iter()
        .map(|field| {
            let field_ident = field.ident.as_ref().unwrap();
            quote!(#field_ident)
        })
        .collect::<Vec<_>>();
    trace!("fields:\n{:#?}", quote!(#(#fields)*).to_string());
    let println_result_ident = Ident::new("result", proc_macro::Span::call_site().into());
    let println_field_steps = raw_fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let format_str = format!("{}.{{}} = {{}}\n", item_label);
        quote!(
            #println_result_ident.push_str(&format!(#format_str, stringify!(#field_ident), tmr::ToToml::to_toml(&self.#field_ident)));
        )
    }).collect::<Vec<_>>();
    trace!(
        "println_steps:\n{:#?}",
        quote!(#(#println_field_steps)*).to_string()
    );
    let result = quote!(
        impl #struct_ident {
            fn new() -> Self {
                #(#field_assigns)*
                Self {
                    #(#fields),*
                }
            }

            fn println(&self) -> String {
                let mut #println_result_ident = String::new();
                #(#println_field_steps)*
                #println_result_ident
            }
        }
    );
    trace!("result:\n{:#?}", result.to_string());
    result.into()
}
