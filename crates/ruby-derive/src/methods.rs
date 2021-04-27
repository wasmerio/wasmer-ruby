use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Colon2, *};

pub fn entry(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let impl_block = parse_macro_input!(input as ItemImpl);

    let ty_name = match *impl_block.self_ty {
        Type::Path(TypePath { qself: None, path }) if path.get_ident().is_some() => {
            path.get_ident().unwrap().clone()
        }

        _ => panic!("`impl` block must target a simple identifier, e.g. `impl T`"),
    };

    let ruby_ty_name = Ident::new(&format!("Ruby{}", ty_name), ty_name.span());

    let mut ruby_items = Vec::with_capacity(impl_block.items.len());

    for item in impl_block.items {
        match item {
            ImplItem::Method(method) => {
                if !method.attrs.is_empty() {
                    panic!("Methods in `impl` block do not support attributes for the moment");
                }

                let method_name = method.sig.ident.to_string();

                if method.sig.constness.is_some() {
                    panic!("Method `{}` cannot be `const`", method_name);
                }

                if method.sig.asyncness.is_some() {
                    panic!("Method `{}` cannot be `async`", method_name);
                }

                if method.sig.unsafety.is_some() {
                    panic!("Method `{}` cannot be `unsafe`", method_name);
                }

                if method.sig.abi.is_some() {
                    panic!("Method `{}` cannot have a different ABI", method_name);
                }

                if !method.sig.generics.params.is_empty() {
                    panic!("Method `{}` cannot be generic", method_name);
                }

                if method.sig.variadic.is_some() {
                    panic!("Method `{}` cannot be variadic", method_name);
                }

                let ruby_method_name = &method.sig.ident;
                let ruby_method_block = &method.block;
                let ruby_method_visibility = &method.vis;

                let mut need_self = false;
                let mut need_mut_self = false;

                let ruby_arguments_parsing = {
                    let (ruby_input_names, ruby_input_types): (
                        Vec<Ident>,
                        Vec<Punctuated<PathSegment, Colon2>>,
                    ) = method
                        .sig
                        .inputs
                        .iter()
                        .filter_map(|input| match input {
                            FnArg::Receiver(Receiver { mutability, .. }) => {
                                need_self = true;
                                need_mut_self = mutability.is_some();

                                None
                            }
                            FnArg::Typed(PatType { pat, ty, .. }) => match (&**pat, &**ty) {
                                (
                                    Pat::Ident(ident),
                                    Type::Path(TypePath {
                                        qself: None,
                                        path: Path { segments: ty, .. },
                                    }),
                                ) => Some((ident.ident.clone(), ty.clone())),
                                _ => panic!(
                                    "Typed input has an unsupported form (method `{}`)",
                                    method_name
                                ),
                            },
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
                                                            Some(&format!(concat!("Argument #{} (`", stringify!( #ruby_input_types ), "`) of method `", stringify!( #ruby_method_name ), "` is missing"), argument_nth)),
                                                        )
                                                    })
                                                    .and_then(|argument| {
                                                        <rutie::AnyObject as rutie::Object>
                                                            ::try_convert_to::< #ruby_input_types >(argument)
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
                        }
                    }
                };

                let ruby_input_receiver = if need_mut_self {
                    quote! { mut _slf: #ruby_ty_name }
                } else {
                    quote! { _slf: #ruby_ty_name }
                };

                let ruby_output = match method.sig.output {
                    ReturnType::Type(_, ty) => match *ty {
                        Type::Path(TypePath {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments,
                            }
                        }) if segments.first().unwrap().ident.to_string() == "RubyResult" => {
                            match &segments.first().unwrap().arguments {
                                PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                    args,
                                    ..
                                }) => match args.first().unwrap() {
                                    GenericArgument::Type(ty) => ty.clone(),
                                    _ => panic!("Method has not well-formed rerturned type, expect `RubyResult<T>` where `T` is a type"),
                                },
                                _ => panic!("Method has not well-formed returned type, expect `RubyResult<T>`"),
                            }
                        }
                        _ => panic!("Method must wrap their output type inside `RubyResult<T>`"),
                    },
                    _ => panic!("Method must have an output of the form `RubyResult<T>`"),
                };

                let ruby_method_block = if need_self {
                    rename_self(ruby_method_block.into_token_stream())
                } else {
                    ruby_method_block.into_token_stream()
                };

                let ruby_block_visibility = if need_mut_self {
                    quote! { mut }
                } else {
                    quote! {}
                };

                ruby_items.push(quote! {
                    #[allow(improper_ctypes_definitions)] // Not ideal but that's how Rutie works.
                    #ruby_method_visibility extern "C" fn #ruby_method_name(
                        argc: rutie::types::Argc,
                        argv: *const rutie::AnyObject,
                        #ruby_input_receiver,
                    ) -> #ruby_output {
                        #ruby_arguments_parsing

                        let #ruby_block_visibility block = || -> Result<_, rutie::AnyException> {
                            #ruby_method_block
                        };

                        match block() {
                            Ok(x) => x,
                            Err(e) => {
                                rutie::VM::raise_ex(e);
                                unreachable!()
                            }
                        }
                    }
                });
            }

            _ => panic!("`impl` block only supports methods for the moment"),
        }
    }

    (quote! {
        pub(crate) mod ruby {
            use super::*;

            #(#ruby_items)*
        }
    })
    .into()
}

fn rename_self(stream: TokenStream) -> TokenStream {
    let mut output_stream = TokenStream::new();

    output_stream.extend(
        stream
            .into_iter()
            .map(|token| match token {
                TokenTree::Group(group) => {
                    TokenTree::Group(Group::new(group.delimiter(), rename_self(group.stream())))
                }
                TokenTree::Ident(ident) => {
                    if ident.to_string() == "self" {
                        TokenTree::Ident(Ident::new("_slf", ident.span()))
                    } else {
                        TokenTree::Ident(ident)
                    }
                }
                TokenTree::Punct(punct) => TokenTree::Punct(punct),
                TokenTree::Literal(literal) => TokenTree::Literal(literal),
            })
            .collect::<Vec<TokenTree>>(),
    );

    output_stream
}
