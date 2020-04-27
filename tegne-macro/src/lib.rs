use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::env;
use std::process::Command;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::LitByteStr;
use syn::LitStr;
use syn::Result;
use syn::Token;

#[proc_macro]
pub fn include_shader(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as PathList)
        .paths
        .iter()
        .map(|p| p.value())
        .collect::<String>();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("crate must be built with cargo");
    let output = Command::new("glslc")
        .arg(format!("{}/{}", manifest_dir, path))
        .arg("-o-")
        .arg("-std=450")
        .arg("--target-env=vulkan1.1")
        .output()
        .expect("cannot include shader, glslc not installed (part of the Vulkan SDK)");

    if !output.status.success() {
        let err = std::str::from_utf8(&output.stderr)
            .expect("invalid utf8")
            .to_owned();
        panic!(err);
    }

    let source = LitByteStr::new(&output.stdout, Span::call_site());

    (quote! {#source}).into()
}

struct PathList {
    paths: Punctuated<LitStr, Token![,]>,
}

impl Parse for PathList {
    fn parse(input: ParseStream) -> Result<Self> {
        type Inner = Punctuated<LitStr, Token![,]>;
        let paths = Inner::parse_separated_nonempty(&input)?;
        Ok(Self { paths })
    }
}
