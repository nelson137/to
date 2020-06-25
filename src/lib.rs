extern crate proc_macro;

#[macro_use]
extern crate syn;

extern crate quote;

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Ident, Lit, Path, Variant,
};

mod attrs;
use attrs::ToLangAttr;

#[proc_macro_derive(ToLangs, attributes(to_lang))]
#[proc_macro_error]
pub fn to_langs(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    match input.data {
        Data::Enum(e) => to_langs_enum(&input.ident, &e.variants),
        _ => abort!(input.ident, "ToLangs only supports enums"),
    }
}

fn to_langs_enum(
    enum_name: &Ident,
    variants_input: &Punctuated<Variant, Token![,]>,
) -> TokenStream {
    let mut compile_func_vars: Vec<Ident> = Vec::new();
    let mut compile_func_fns: Vec<Path> = Vec::new();
    let mut extensions_exts: Vec<Lit> = Vec::new();
    let mut extensions_vars: Vec<Ident> = Vec::new();

    for var in variants_input.iter() {
        for attr in &var.attrs {
            if attr.path.is_ident("to_lang") {
                match attr.parse_args::<ToLangAttr>() {
                    Ok(to_lang_attr) => {
                        compile_func_vars.push(var.ident.clone());
                        compile_func_fns.push(to_lang_attr.compile_func);
                        for ext in to_lang_attr.extensions.into_iter() {
                            extensions_vars.push(var.ident.clone());
                            extensions_exts.push(ext);
                        }
                    }
                    Err(err) => abort!(attr, err),
                }
            }
        }
    }

    (quote! {
        impl #enum_name {
            pub fn determine(file: &PathBuf) -> Option<Self> {
                use #enum_name::*;

                let extensions: &'static [&'static str] = &[ #( #extensions_exts ),* ];
                let variants: &'static [&'static Self] = &[ #( &#extensions_vars ),* ];

                let extension = file.extension()?.to_str()?;
                for i in 0..variants.len() {
                    if extensions[i] == extension {
                        return Some(*variants[i]);
                    }
                }

                None
            }

            pub fn compile(&self, infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
                use #enum_name::*;
                use std::mem::discriminant;

                let variants: &'static [&'static Self] = &[ #( &#compile_func_vars ),* ];
                let functions: &'static [fn(&PathBuf, &PathBuf) -> Result<Vec<PathBuf>, String>] = &[ #( #compile_func_fns ),* ];

                for i in 0..variants.len() {
                    if discriminant(variants[i]) == discriminant(self) {
                        return functions[i](infile, outfile);
                    }
                }

                Err(format!("No compiler implemented for language: {:?}", self))
            }
        }
    }).into()
}
