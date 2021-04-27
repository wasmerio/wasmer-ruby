use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Data, DataStruct, DeriveInput, Generics, Ident};

#[proc_macro_derive(RubyClass)]
pub fn derive_ruby(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse(input).unwrap();

    match derive_input.data {
        Data::Struct(ref struct_data) => {
            derive_rubyclass_for_struct(&derive_input.ident, struct_data, &derive_input.generics)
        }

        Data::Enum(_) => panic!("enums are not yet supported"),

        Data::Union(_) => panic!("unions are not yet supported"),
    }
}

fn derive_rubyclass_for_struct(
    struct_name: &Ident,
    _data: &DataStruct,
    generics: &Generics,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let span = struct_name.span();
    let wrapper_struct_name = Ident::new(&format!("{}Wrapper", struct_name), span);
    let wrapper_const_name = Ident::new(
        &format!("{}_WRAPPER", struct_name.to_string().to_uppercase()),
        span,
    );
    let ruby_struct_name = Ident::new(&format!("Ruby{}", struct_name), span);

    (quote! {
        use rutie::{wrappable_struct, typed_data::DataTypeWrapper};

        wrappable_struct!(#struct_name, #wrapper_struct_name, #wrapper_const_name);

        rutie::class!(#ruby_struct_name);

        impl #impl_generics #struct_name #ty_generics
        #where_clause
        {
            pub(crate) fn wrap(this: Self) -> rutie::AnyObject {
                rutie::Module::from_existing("Wasmer")
                    .get_nested_class(stringify!(#struct_name))
                    .wrap_data(this, &*#wrapper_const_name)
            }
        }

        impl #ruby_struct_name {
            pub(crate) fn unwrap(&self) -> &#struct_name {
                use rutie::Object;

                self.get_data(&*#wrapper_const_name)
            }
        }

        impl #impl_generics rutie::VerifiedObject for #ruby_struct_name #ty_generics
        #where_clause
        {
            fn is_correct_type<T>(object: &T) -> bool
            where T: rutie::Object
            {
                object.class() == rutie::Module::from_existing("Wasmer").get_nested_class(stringify!(#struct_name))
            }

            fn error_message() -> &'static str {
                concat!("Error converting to `", stringify!(#struct_name), "`")
            }
        }
    })
    .into()
}
