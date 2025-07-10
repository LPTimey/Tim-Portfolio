#![allow(non_snake_case)]
extern crate proc_macro;

use std::collections::HashSet;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    FnArg, GenericArgument, ItemFn, Lifetime, Pat, PatIdent, PathArguments, Type, TypePath,
    parse_macro_input, spanned::Spanned,
};

fn extract_generics_and_lifetimes(ty: &Type) -> (Vec<Lifetime>, Vec<Type>) {
    let mut lifetimes = Vec::new();
    let mut generics = Vec::new();

    match ty {
        Type::Reference(ty_ref) => {
            // & or &'a
            if let Some(lt) = &ty_ref.lifetime {
                lifetimes.push(lt.clone());
            }
            let (lts, gens) = extract_generics_and_lifetimes(&ty_ref.elem);
            lifetimes.extend(lts);
            generics.extend(gens);
        }
        Type::Path(TypePath { path, .. }) => {
            for segment in &path.segments {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        match arg {
                            GenericArgument::Lifetime(lt) => {
                                lifetimes.push(lt.clone());
                            }
                            GenericArgument::Type(inner_ty) => {
                                let (lts, gens) = extract_generics_and_lifetimes(inner_ty);
                                lifetimes.extend(lts);
                                generics.extend(gens);
                                generics.push(inner_ty.clone()); // add the generic itself
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        _ => {
            // Optional: handle other types like Tuple, Array, etc. if needed
        }
    }

    (lifetimes, generics)
}

#[proc_macro_attribute]
pub fn with_props(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Funktion parsen
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let vis = &input_fn.vis;
    let vis = syn::Visibility::Public(syn::token::Pub {
        span: input_fn.vis.span(),
    });

    // Props-Struct-Name erzeugen, z.B. MarkupProps
    let props_name = format_ident!("{}Props", fn_name.to_string().to_case(Case::UpperCamel));

    // Sammle alle Parameter in Felder fürs Struct
    let mut fields = Vec::new();
    let mut bindings = Vec::new();

    let mut lifetimes = Vec::new();
    let mut generics = Vec::new();

    let mut seen_lifetimes = HashSet::new();
    let mut seen_generics = HashSet::new();

    for arg in &input_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(PatIdent { ident, .. }) = &*pat_type.pat {
                let name = ident;
                let ty = &pat_type.ty;
                let (lifetime, generic) = extract_generics_and_lifetimes(ty.as_ref());
                fields.push(quote! { pub #name: #ty });
                lifetimes.extend(lifetime);
                generics.extend(generic);
                lifetimes.retain(|lt| seen_lifetimes.insert(lt.ident.to_string()));
                generics.retain(|ty| seen_generics.insert(quote!(#ty).to_string()));
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
        .push(syn::parse_quote! { props: #props_name });

    // Erzeuge Destrukturierung am Anfang der Funktion
    let block = &input_fn.block;
    input_fn.block = syn::parse_quote! {{
        let #props_name { #(#bindings),* } = props;
        #block
    }};

    let generics_list = {
        let mut params = Vec::new();
        params.extend(lifetimes.iter().map(|lt| quote!(#lt)));
        params.extend(generics.iter().map(|ty| quote!(#ty)));
        if !params.is_empty() {
            quote! { <#(#params),*> }
        } else {
            quote! {}
        }
    };
    // Erzeuge den Props-Struct-Code
    let expanded = quote! {
        #input_fn

        #vis struct #props_name #generics_list  {
            #(#fields),*
        }
    };

    expanded.into()
}
