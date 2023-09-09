use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Ident, ItemImpl,
};
use tracing::{info_span, trace};

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

// proc macro attributes
#[proc_macro_attribute]
pub fn package(attr: TokenStream, item: TokenStream) -> TokenStream {
    tracing_setup();
    let _enter = info_span!("package").entered();
    let _attr = parse_macro_input!(attr as AttributePackage);
    let item = parse_macro_input!(item as ItemImpl);
    let struct_ty = item.self_ty.as_ref();
    let println_result_ident = Ident::new("result", proc_macro::Span::call_site().into());
    let println_fn_steps = item
        .items
        .iter()
        .map(|item| match item {
            syn::ImplItem::Fn(method) => {
                let method_ident = &method.sig.ident;
                quote!(result.push_str(&format!("package.{} = {:?}\n", stringify!(#method_ident), self.#method_ident()));)
            }
            _ => panic!("Only methods are allowed in impl blocks"),
        })
        .collect::<Vec<_>>();
    trace!(
        "println_fn_steps:\n{:#?}",
        quote!(#(#println_fn_steps)*).to_string()
    );
    let method_definitions = item
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

struct AttributePackage {}

impl Parse for AttributePackage {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(AttributePackage {})
    }
}

#[proc_macro_derive(Package, attributes(value))]
pub fn derive_package(input: TokenStream) -> TokenStream {
    tracing_setup();
    let _enter = info_span!("derive_package").entered();
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident;
    let raw_fields = match input.data {
        syn::Data::Struct(data_struct) => data_struct.fields,
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
            quote!(let #field_ident = #field_value.to_string();)
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
        quote!(#println_result_ident.push_str(&format!("package.{} = {:?}\n", stringify!(#field_ident), self.#field_ident));)
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

enum ItemCargoToml {
    Package(PackageStruct),
}

impl Parse for ItemCargoToml {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(ItemCargoToml::Package(PackageStruct(IdentExt::parse_any(
                input,
            )?)))
        } else {
            Err(lookahead.error())
        }
    }
}

struct PackageStruct(Ident);

impl PackageStruct {
    fn name(&self) -> &Ident {
        &self.0
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
        .map(|item| match item {
            ItemCargoToml::Package(package) => {
                let struct_name = package.name();
                let struct_bound = Ident::new(
                    stringify!(name).to_owned().to_lowercase().as_ref(),
                    proc_macro::Span::call_site().into(),
                );
                quote!(
                    let #struct_bound = #struct_name::new();
                    #cargo_toml_contents_ident.push_str(&format!("{}", #struct_bound.println()));
                    #cargo_toml_contents_ident.push_str(&format!("{}", #struct_bound.println_fn()));
                )
            }
        })
        .collect::<Vec<_>>();
    trace!(
        "cargo_toml_build_string_steps:\n{:#?}",
        quote!(#(#cargo_toml_build_string_steps)*).to_string()
    );
    let cargo_toml_file_ident = Ident::new("cargo_toml_file", proc_macro::Span::call_site().into());
    quote!(
        fn main() {
            let mut #cargo_toml_file_ident = std::fs::File::create("Cargo.toml").unwrap();
            let #cargo_toml_contents_ident = {
                let mut #cargo_toml_contents_ident = String::new();
                #(#cargo_toml_build_string_steps)*
                #cargo_toml_contents_ident
            };
            std::io::Write::write_all(&mut #cargo_toml_file_ident, #cargo_toml_contents_ident.as_bytes()).unwrap();
        }
    )
    .into()
}
