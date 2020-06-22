extern crate proc_macro;

#[macro_use]
extern crate syn;

extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, Data, DeriveInput, Ident, Lit, Meta,
    NestedMeta, Path, Variant,
};

#[proc_macro_derive(SupportedLangs, attributes(compile_func, extensions))]
pub fn supported_langs(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    match input.data {
        Data::Enum(ref e) => supported_langs_enum(name, &e.variants, &input.attrs),
        _ => panic!("SupportedLangs only supports enums"),
    }
}

fn add_compile_func(entries: &mut Vec<Path>, attr: &Meta) {
    match attr {
        Meta::List(l) => {
            if l.nested.len() != 1 {
                panic!("compile_func attr can only accept one argument");
            }
            match &l.nested[0] {
                NestedMeta::Meta(m) => match m {
                    Meta::Path(p) => entries.push(p.clone()),
                    _ => panic!("compile_func attr requires paths"),
                },
                _ => panic!("compile_func attr requires a function"),
            }
        }
        _ => panic!("compile_func attr must be like #[compile_func(...)]"),
    };
}

fn add_extension_entries(
    entries_ext: &mut Vec<Lit>,
    entries_var: &mut Vec<Ident>,
    var: &Ident,
    attr: &Meta,
) {
    match attr {
        Meta::List(l) => {
            for ext in &l.nested {
                match ext {
                    NestedMeta::Lit(l) => {
                        entries_ext.push(l.clone());
                        entries_var.push(var.clone());
                    }
                    _ => panic!("extensions attr requires string literals"),
                }
            }
        }
        _ => panic!("extensions attr must be like #[extensions(...)]"),
    }
}

fn supported_langs_enum(
    enum_name: &Ident,
    variants_input: &Punctuated<Variant, Token![,]>,
    _attrs: &[Attribute],
) -> TokenStream {
    let mut variants: Vec<Ident> = Vec::new();
    let mut compile_funcs: Vec<Path> = Vec::new();
    let mut extensions_ext: Vec<Lit> = Vec::new();
    let mut extensions_var: Vec<Ident> = Vec::new();

    for var in variants_input.iter() {
        for attr in &var.attrs {
            let meta = attr.parse_meta().unwrap();
            if attr.path.is_ident("compile_func") {
                variants.push(var.ident.clone());
                add_compile_func(&mut compile_funcs, &meta);
            } else if attr.path.is_ident("extensions") {
                add_extension_entries(&mut extensions_ext, &mut extensions_var, &var.ident, &meta);
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
                    #( Self::#variants => #compile_funcs ),*
                })(infile, outfile)
            }
        }
    }).into()
}
