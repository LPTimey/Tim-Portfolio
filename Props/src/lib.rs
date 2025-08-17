#![allow(non_snake_case)]

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, FnArg, GenericParam, ItemFn, Pat, PatIdent};

#[proc_macro_attribute]
pub fn with_props(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    let derives: proc_macro2::TokenStream = attr.into();
    // Funktion parsen
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_generics = &input_fn.sig.generics;
    let vis = syn::Visibility::Public(syn::token::Pub {
        span: input_fn.vis.span(),
    });

    // Props-Struct-Name erzeugen, z.B. MarkupProps
    let props_name = format_ident!("{}Props", fn_name.to_string().to_case(Case::UpperCamel));
    let type_param_ident_s = fn_generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(ty_param) = param {
                Some(&ty_param.ident)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let generics_tokens = if type_param_ident_s.is_empty() {
        quote! {}
    } else {
        quote! { <#(#type_param_ident_s),*> } // generische Parameter mit <...>
    };

    // Sammle alle Parameter in Felder fürs Struct
    let mut fields = Vec::new();
    let mut bindings = Vec::new();

    for arg in &input_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(PatIdent { ident, .. }) = &*pat_type.pat {
                let name = ident;
                let ty = &pat_type.ty;
                fields.push(quote! { pub #name: #ty });
                bindings.push(quote! { #name });
            }
        } else {
            // evtl. Receiver (self) behandeln, hier nicht erwartet
        }
    }

    // Ersetze Funktionsparameter durch `props: PropsName`
    input_fn.sig.inputs.clear();
    input_fn
        .sig
        .inputs
        .push(syn::parse_quote! { props: #props_name #generics_tokens });

    // Destructure Props für die Funktion
    let block = &input_fn.block;
    input_fn.block = syn::parse_quote! {{
        let #props_name { #(#bindings),* } = props;
        #block
    }};

    // Erzeuge den Props-Struct-Code
    let expanded = quote! {
        #input_fn

        #[derive(#derives)]
        #vis struct #props_name #fn_generics  {
            #(#fields),*
        }
    };
    expanded.into()
}
