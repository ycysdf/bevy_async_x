use proc_macro::TokenStream;

use quote::quote;
use syn::{
    FnArg, ItemFn, parse_macro_input
    , parse_quote, punctuated::Punctuated, Token,
};

#[proc_macro_attribute]
pub fn async_system(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemFn);

    {
        let vis = ast.vis;
        let metas = ast.attrs.iter().map(|n|&n.meta);
        let mut signature = ast.sig;
        let block = ast.block;

        let params = signature
            .inputs
            .iter()
            .map(|n| {
                let FnArg::Typed(pat) = n else { unreachable!() };
                pat.pat.clone()
            })
            .collect::<Punctuated<_, Token![,]>>();
        signature.inputs.push(parse_quote! {
            mut future_state: Local<bevy_async_x::FutureState>
        });
        signature.asyncness = None;
        quote! {
            #(#[#metas])*
            #vis #signature {
                let params = (#params);
                bevy_async_x::tick_future(params,&mut *future_state, |__p| async {
                    let (#params) = __p;
                    #block
                })
            }
        }
    }
    .into()
}