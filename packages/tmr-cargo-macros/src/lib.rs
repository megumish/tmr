use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Ident, ItemImpl,
};
use tracing::{info_span, trace};

mod item;

fn tracing_setup() {
    if tracing::dispatcher::has_been_set() {
        return;
    }
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter("tmr_macros=trace")
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::ENTER
                | tracing_subscriber::fmt::format::FmtSpan::EXIT,
        )
        .pretty()
        .init();
}

fn println_fn_ident() -> Ident {
    Ident::new("println_fn", proc_macro::Span::call_site().into())
}

fn println_ident() -> Ident {
    Ident::new("println", proc_macro::Span::call_site().into())
}

// proc macro attributes
#[proc_macro_attribute]
pub fn item(attr: TokenStream, item: TokenStream) -> TokenStream {
    tracing_setup();
    let _enter = info_span!("item").entered();
    let attr = parse_macro_input!(attr as AttributeItem);
    let impl_definition = parse_macro_input!(item as ItemImpl);
    let result = item::item(&attr.value.value(), &impl_definition);
    result.into()
}

struct AttributeItem {
    value: syn::LitStr,
}

impl Parse for AttributeItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse()?;
        Ok(AttributeItem { value })
    }
}

#[proc_macro_derive(Item, attributes(key, value, values, route))]
pub fn derive_package(input: TokenStream) -> TokenStream {
    tracing_setup();
    let _enter = info_span!("derive_iteme").entered();
    let input = parse_macro_input!(input as DeriveInput);
    let key = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("key"))
        .unwrap()
        .parse_args::<syn::LitStr>()
        .unwrap();
    let result = item::derive_item(&key.value(), &input);
    result.into()
}

struct ItemsCargoToml(Vec<ItemCargoToml>);

impl IntoIterator for ItemsCargoToml {
    type Item = ItemCargoToml;
    type IntoIter = std::vec::IntoIter<ItemCargoToml>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Parse for ItemsCargoToml {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(ItemsCargoToml(items))
    }
}

struct ItemCargoToml(Ident);

impl Parse for ItemCargoToml {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name = input.parse()?;
        let _: Option<syn::Token![,]> = input.parse()?;
        Ok(ItemCargoToml(struct_name))
    }
}

impl ItemCargoToml {
    fn name(&self) -> Ident {
        self.0.clone()
    }
}

#[proc_macro]
pub fn cargo_toml(input: TokenStream) -> TokenStream {
    tracing_setup();
    let _enter = info_span!("cargo_toml").entered();
    let items = parse_macro_input!(input as ItemsCargoToml);
    let cargo_toml_contents_ident =
        Ident::new("cargo_toml_contents", proc_macro::Span::call_site().into());
    let cargo_toml_build_string_steps = items
        .into_iter()
        .map(|item| {
            let struct_name = item.name();
            let struct_bound = Ident::new(
                stringify!(name).to_owned().to_lowercase().as_ref(),
                proc_macro::Span::call_site().into(),
            );
            let result = quote!(
                let #struct_bound = #struct_name::new();
                #cargo_toml_contents_ident.push_str(&format!("{}", #struct_bound.println()));
                #cargo_toml_contents_ident.push_str(&format!("{}", #struct_bound.println_fn()));
            );
            trace!("cargo_toml_build_string_step:\n{:#?}", result.to_string());
            result
        })
        .collect::<Vec<_>>();
    trace!(
        "cargo_toml_build_string_steps:\n{:#?}",
        quote!(#(#cargo_toml_build_string_steps)*).to_string()
    );
    let cargo_toml_file_ident = Ident::new("cargo_toml_file", proc_macro::Span::call_site().into());
    let cargo_toml_doc_ident = Ident::new("doc", proc_macro::Span::call_site().into());
    quote!(
        fn main() {
            let mut #cargo_toml_file_ident = std::fs::File::create("Cargo.toml").unwrap();
            let #cargo_toml_contents_ident = {
                let mut #cargo_toml_contents_ident = String::new();
                #(#cargo_toml_build_string_steps)*
                let mut #cargo_toml_doc_ident = #cargo_toml_contents_ident.parse::<toml_edit::Document>().unwrap();
                #cargo_toml_doc_ident.fmt();
                #cargo_toml_doc_ident.sort_values();
                #cargo_toml_doc_ident.to_string()
            };
            std::io::Write::write_all(&mut #cargo_toml_file_ident, #cargo_toml_contents_ident.as_bytes()).unwrap();
        }
    )
    .into()
}
