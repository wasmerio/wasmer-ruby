mod rubyclass;

#[proc_macro_attribute]
pub fn rubyclass(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    rubyclass::rubyclass(attr, input)
}
