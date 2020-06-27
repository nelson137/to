use proc_macro_error::abort;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, Lit, Path,
};

type CompileFunc = Path;
type Extensions = Punctuated<Lit, Token![,]>;

pub struct ToLangAttr {
    pub compile_func: CompileFunc,
    pub extensions: Extensions,
}

impl Parse for ToLangAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut compile_func: syn::Result<CompileFunc> =
            Err(input.error("missing required parameter: compile_func"));
        let mut extensions: syn::Result<Extensions> =
            Err(input.error("missing required parameter: extensions"));

        while !input.is_empty() {
            let name: Ident = match input.parse() {
                Ok(i) => i,
                Err(err) => abort!(input.span(), err),
            };
            let name_str = name.to_string();

            match &*name_str {
                "compile_func" => compile_func = input.call(parse_compile_func),
                "extensions" => extensions = input.call(parse_extensions),
                _ => abort!(name, format!("unexpected attribute: {}", name_str)),
            }

            // Parse comma after sub-attribute
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(ToLangAttr {
            compile_func: compile_func?,
            extensions: extensions?,
        })
    }
}

fn parse_compile_func(input: ParseStream) -> syn::Result<CompileFunc> {
    if let Err(err) = input.parse::<Token![=]>() {
        abort!(input.span(), err);
    }
    input.parse()
}

fn parse_extensions(input: ParseStream) -> syn::Result<Extensions> {
    let lookahead = input.lookahead1();
    if lookahead.peek(token::Paren) {
        let exts_content;
        parenthesized!(exts_content in input);
        exts_content.parse_terminated(Lit::parse)
    } else {
        abort!(input.span(), lookahead.error());
    }
}
