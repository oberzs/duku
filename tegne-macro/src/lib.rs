use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::process::Command;
use syn::parse_macro_input;
use syn::LitByteStr;
use syn::LitStr;

#[proc_macro]
pub fn include_shader(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr);

    let output = Command::new("glslc")
        .arg(format!("{}", path.value()))
        .arg("-o-")
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
