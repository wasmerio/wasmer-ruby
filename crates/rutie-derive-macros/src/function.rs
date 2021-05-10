use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Colon2, *};

pub fn entry(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let function = parse_macro_input!(input as ItemFn);

    if !function.attrs.is_empty() {
        panic!("Functions in `#[rubyfunction]` do not support attributes for the moment");
    }

    let function_name = function.sig.ident.to_string();

    if function.sig.constness.is_some() {
        panic!("Function `{}` cannot be `const`", function_name);
    }

    if function.sig.asyncness.is_some() {
        panic!("Function `{}` cannot be `async`", function_name);
    }

    if function.sig.unsafety.is_some() {
        panic!("Function `{}` cannot be `unsafe`", function_name);
    }

    if function.sig.abi.is_some() {
        panic!("Function `{}` cannot have a different ABI", function_name);
    }

    if !function.sig.generics.params.is_empty() {
        panic!("Function `{}` cannot be generic", function_name);
    }

    if function.sig.variadic.is_some() {
        panic!("Function `{}` cannot be variadic", function_name);
    }

    let ruby_function_name = &function.sig.ident;
    let ruby_function_block = &function.block;
    let ruby_function_visibility = &function.vis;

    let ruby_arguments_parsing = {
        let (ruby_input_names, ruby_input_types): (
            Vec<Ident>,
            Vec<Punctuated<PathSegment, Colon2>>,
        ) = function
            .sig
            .inputs
            .iter()
            .filter_map(|input| match input {
                FnArg::Typed(PatType { pat, ty, .. }) => match (&**pat, &**ty) {
                    (
                        Pat::Ident(ident),
                        Type::Reference(TypeReference { elem, .. }),
                    ) => match &**elem {
                        Type::Path(TypePath {
                            qself: None,
                            path: Path { segments: ty, .. },
                        }) => Some((ident.ident.clone(), ty.clone())),
                        _ => panic!(
                            "Typed input has an unsupported form (function `{}`)",
                            function_name
                        ),
                    },
                    _ => panic!(
                        "Typed input has an unsupported form (function `{}`), it must be a reference type",
                        function_name
                    ),
                },

                FnArg::Receiver(..) => unreachable!(),
            })
            .unzip();

        if ruby_input_names.is_empty() {
            quote! {}
        } else {
            quote! {
                let ( #( #ruby_input_names ),* ) =
                {
                    let arguments = rutie::util::parse_arguments(argc, argv);
                    let mut argument_nth = 0;

                    (
                        #(
                            {
                                let argument = arguments
                                    .get(argument_nth)
                                    .ok_or_else(|| {
                                        <rutie::AnyException as rutie::Exception>::new(
                                            "ArgumentError",
                                            Some(&format!(concat!("Argument #{} (`", stringify!( #ruby_input_types ), "`) of function `", stringify!( #ruby_function_name ), "` is missing"), argument_nth)),
                                        )
                                    })
                                    .and_then(|argument| {
                                        <rutie::AnyObject as rutie::Object>
                                            ::try_convert_to::<< #ruby_input_types as rutie_derive::ClassInfo>::RubyClass>(argument)
                                    })
                                    .unwrap_or_else(|error| {
                                        rutie::VM::raise_ex(error);
                                        unreachable!()
                                    });

                                argument_nth += 1;

                                argument
                            }
                        ),*
                    )
                };

                let ( #( #ruby_input_names ),* ) =
                    (
                        #(
                            rutie_derive::UpcastRubyClass::<
                                    <
                                            < #ruby_input_types as rutie_derive::ClassInfo>::RubyClass as rutie_derive::ClassInfo
                                            >::Class
                                    >::upcast(&#ruby_input_names)
                        ),*
                    );
            }
        }
    };

    let ruby_output = match function.sig.output {
        ReturnType::Type(_, ty) => match *ty {
            Type::Path(TypePath {
                qself: None,
                path:
                    Path {
                        leading_colon: None,
                        segments,
                    },
            }) if segments.first().unwrap().ident.to_string() == "RubyResult" => {
                match &segments.first().unwrap().arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }) => match args.first().unwrap() {
                        GenericArgument::Type(ty) => ty.clone(),
                        _ => panic!("Function has not well-formed rerturned type, expect `RubyResult<T>` where `T` is a type"),
                    },
                    _ => panic!("Function has not well-formed returned type, expect `RubyResult<T>`"),
                }
            }
            _ => panic!("Function must wrap their output type inside `RubyResult<T>`"),
        },
        _ => panic!("Function must have an output of the form `RubyResult<T>`"),
    };

    (quote! {
        #[allow(improper_ctypes_definitions)] // Not ideal but that's how Rutie works.
        #ruby_function_visibility extern "C" fn #ruby_function_name(
            argc: rutie::types::Argc,
            argv: *const rutie::AnyObject,
            _: rutie::AnyObject,
        ) -> #ruby_output {
            #ruby_arguments_parsing

            let block = || -> Result<_, rutie::AnyException> {
                #ruby_function_block
            };

            match block() {
                Ok(x) => x,
                Err(e) => {
                    rutie::VM::raise_ex(e);
                    unreachable!()
                }
            }
        }
    })
    .into()
}
