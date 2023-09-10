use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, ItemImpl};
use tracing::trace;

#[tracing::instrument(level = "info", skip(impl_definition))]
pub fn item(item_label: &str, impl_definition: &ItemImpl) -> TokenStream {
    let struct_ty = impl_definition.self_ty.as_ref();
    let println_fn_ident = super::println_fn_ident();
    let println_result_ident = Ident::new("result", proc_macro::Span::call_site().into());
    let println_fn_steps = impl_definition
        .items
        .iter()
        .map(|item| match item {
            syn::ImplItem::Fn(method) => 
            {
                let method_ident = &method.sig.ident;
                if let Some(attr_value) = method
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("route"))
                {
                    let route = attr_value.parse_args::<syn::LitStr>().unwrap();
                    let format_str = format!("{}.{} = {{}}\n", &item_label, route.value());
                    quote!(
                        result.push_str(&format!(#format_str, tmr::ToToml::to_toml(&self.#method_ident())));
                    )
                } else {
                    let format_str = format!("{}.{{}} = {{}}\n", &item_label);
                    quote!(
                        result.push_str(&format!(#format_str, stringify!(#method_ident), tmr_cargo::ToToml::to_toml(&self.#method_ident())));
                    )
                }},

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
            syn::ImplItem::Fn(method) => {
                let method_ident = &method.sig.ident;
                let method_return_type = &method.sig.output;
                let method_contents = &method.block;
                let method_inputs = &method.sig.inputs;
                quote!(
                    fn #method_ident(#method_inputs) #method_return_type {
                        #method_contents
                    }
                )
            }
            _ => panic!("Only methods are allowed in impl blocks"),
        })
        .collect::<Vec<_>>();
    trace!(
        "method_definitions:\n{:#?}",
        quote!(#(#method_definitions)*).to_string()
    );
    let result = quote!(
        impl #struct_ty {
            fn #println_fn_ident(&self) -> String {
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

#[tracing::instrument(level = "info", skip(input))]
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
            if let Some(attr_value) = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("value"))
            {
                let value = attr_value.parse_args::<syn::LitStr>().unwrap();
                quote!(let #field_ident = #value.try_into().unwrap();)
            } else {
                let values = field
                    .attrs
                    .iter()
                    .filter(|attr| attr.path().is_ident("values"))
                    .map(|attr| attr.parse_args::<syn::LitStr>().unwrap());
                quote!(let #field_ident = (&[#(#values),*][..]).try_into().unwrap();)
            }
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
    let println_ident = super::println_ident();
    let println_result_ident = Ident::new("result", proc_macro::Span::call_site().into());
    let println_field_steps = raw_fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        if let Some(attr_value) = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("route"))
        {
            let route = attr_value.parse_args::<syn::LitStr>().unwrap();
            let format_str = format!("{}.{} = {{}}\n", &item_label, route.value());
            quote!(
                result.push_str(&format!(#format_str, tmr::ToToml::to_toml(&self.#field_ident)));
            )
        } else {
            let format_str = format!("{}.{{}} = {{}}\n", &item_label);
            quote!(
                result.push_str(&format!(#format_str, stringify!(#field_ident),  tmr_cargo::ToToml::to_toml(&self.#field_ident)));
            )
        }
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

            fn #println_ident(&self) -> String {
                let mut #println_result_ident = String::new();
                #(#println_field_steps)*
                #println_result_ident
            }
        }
    );
    trace!("result:\n{:#?}", result.to_string());
    result.into()
}
