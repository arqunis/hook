// ISC License (ISC)
//
// Copyright (c) 2025, Serenity Contributors
//
// Permission to use, copy, modify, and/or distribute this software for any purpose
// with or without fee is hereby granted, provided that the above copyright notice
// and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
// OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
// THIS SOFTWARE.

use proc_macro::TokenStream;

use proc_macro2::Span;

use syn::parse_macro_input;
use syn::{Token, FnArg, ItemFn, Signature, Type, ReturnType, Lifetime};
use syn::spanned::Spanned;
use syn::punctuated::Punctuated;

use quote::quote;

#[proc_macro_attribute]
pub fn hook(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as ItemFn);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = fun;

    let sig_span = sig.span();
    let Signature {
        asyncness,
        ident,
        mut inputs,
        output,
        ..
    } = sig;

    if asyncness.is_none() {
        return syn::Error::new(sig_span, "`async` keyword is missing").to_compile_error().into();
    }

    let output = match output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, t) => quote!(#t),
    };

    populate_lifetime(&mut inputs);

    let result = quote! {
        #(#attrs)*
        #vis fn #ident<'fut>(#inputs) -> futures::future::BoxFuture<'fut, #output> {
            use futures::future::FutureExt;

            async move {
                #block
            }.boxed()
        }
    };

    result.into()
}

fn populate_lifetime(inputs: &mut Punctuated<FnArg, Token![,]>) {
    for input in inputs {
        if let FnArg::Typed(kind) = input {
            if let Type::Reference(ty) = &mut *kind.ty {
                ty.lifetime = Some(Lifetime::new("'fut", Span::call_site()));
            }
        }
    }
}
