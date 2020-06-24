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
        Data::Enum(ref e) => to_langs_enum(&input.ident, &e.variants),
        _ => abort!(input.ident, "ToLangs only supports enums"),
    }
}

fn to_langs_enum(
    enum_name: &Ident,
    variants_input: &Punctuated<Variant, Token![,]>,
) -> TokenStream {
    let mut compile_funcs_vars: Vec<Ident> = Vec::new();
    let mut compile_funcs_fns: Vec<Path> = Vec::new();
    let mut extensions_ext: Vec<Lit> = Vec::new();
    let mut extensions_var: Vec<Ident> = Vec::new();

    for var in variants_input.iter() {
        for attr in &var.attrs {
            if attr.path.is_ident("to_lang") {
                match attr.parse_args::<ToLangAttr>() {
                    Ok(to_lang_attr) => {
                        compile_funcs_vars.push(var.ident.clone());
                        compile_funcs_fns.push(to_lang_attr.compile_func);
                        for ext in to_lang_attr.extensions.into_iter() {
                            extensions_var.push(var.ident.clone());
                            extensions_ext.push(ext);
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
                let extensions = [ #( #extensions_ext ),* ];
                let variants = [ #( Self::#extensions_var ),* ];

                let ext = file.extension()?.to_str()?;
                for i in 0..extensions.len() {
                    if extensions[i] == ext {
                        return Some(variants[i]);
                    }
                }

                None
            }

            pub fn compile(&self, infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String>{
                (match self {
                    #( Self::#compile_funcs_vars => #compile_funcs_fns ),*
                })(infile, outfile)
            }
        }
    }).into()
}
