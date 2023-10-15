extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, FnArg, ItemFn, Pat, Stmt};

fn translate_inputs<'a>(it: impl Iterator<Item = &'a mut FnArg>) -> Vec<Stmt> {
    let mut out: Vec<Stmt> = vec![];

    it.enumerate()
        .map(|(i, arg)| {
            let FnArg::Typed(arg) = arg else {
                panic!("self is not allowed in guest functions")
            };
            let Pat::Ident(id) = &*arg.pat else {
                panic!("Invalid function declation")
            };
            (i, id.ident.clone(), &mut arg.ty)
        })
        .for_each(|(index, name, ty)| {
            out.push(
                parse_quote!(let #name: #ty = plugin::convert_value::<#ty>(&args.params[#index], #index).unwrap();),
            );
        });

    out
}

#[proc_macro_attribute]
pub fn plugin_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = parse_macro_input!(item as ItemFn);

    let prelude = translate_inputs(item_fn.sig.inputs.iter_mut());

    let fn_name = &item_fn.sig.ident;
    let fn_block = &item_fn.block;
    let output_type = item_fn.sig.output.clone();

    let expanded = quote! {

        #[no_mangle]
        pub unsafe extern "C" fn #fn_name(ptr: *mut u8, len: i32) -> *mut plugin::ReturnValues {
            let slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
            let json_str = match std::str::from_utf8(slice) {
                Ok(s) => s,
                Err(_) => return plugin::serialize_to_return_values(&plugin::ErrorValue{reason:"Failed to convert byte slice to string".to_string()}),
            };

            let args: plugin::InputWrapper = match serde_json::from_str(json_str) {
                Ok(val) => val,
                Err(err) => return plugin::serialize_to_return_values(&plugin::ErrorValue{reason: format!("Failed to deserialize JSON: {}", err).to_string()}),
            };

            #(#prelude)*

            let output = (move || #output_type #fn_block)();

            plugin::serialize_to_return_values(&output)
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn plugin_filter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = parse_macro_input!(item as ItemFn);
    let prelude = translate_inputs(item_fn.sig.inputs.iter_mut());
    let fn_name = &item_fn.sig.ident;
    let fn_block = item_fn.block;
    let output_type = item_fn.sig.output.clone();

    let expanded = quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #fn_name(ptr: *mut u8, len: i32) -> *mut plugin::ReturnValues {
            let slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
            let json_str = match std::str::from_utf8(slice) {
                Ok(s) => s,
                Err(_) => return plugin::serialize_to_return_values(&plugin::ErrorValue{reason:"Failed to convert byte slice to string".to_string()}),
            };

            let args: plugin::InputWrapper = match serde_json::from_str(json_str) {
                Ok(val) => val,
                Err(err) => return plugin::serialize_to_return_values(&plugin::ErrorValue{reason: format!("Failed to deserialize JSON: {}", err).to_string()}),
            };

            #(#prelude)*

            let out = (move || #output_type #fn_block)();

            return plugin::serialize_to_return_values(&out);
        }
    };

    TokenStream::from(expanded)
}