mod class;
mod function;
mod methods;

#[proc_macro_attribute]
pub fn rubyclass(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    class::entry(attr, input)
}

#[proc_macro_attribute]
pub fn rubymethods(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    methods::entry(attr, input)
}

#[proc_macro_attribute]
pub fn rubyfunction(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    function::entry(attr, input)
}
