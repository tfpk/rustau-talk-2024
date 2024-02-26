use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn hide(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.to_string().contains("off") {
        item
    } else {
        println!("Hiding item...");
        "".parse().unwrap()
    }
}

#[proc_macro_derive(Create)]
pub fn create(item: TokenStream) -> TokenStream {
    let ast: syn::ItemStruct = syn::parse(item).unwrap();
    let name = &ast.ident;
    let output = quote! {
        fn create() -> #name {
            #name {
                x: 100,
                y: 100
            }
        }
    }.into();
    output
}

// Use: `make_answer!();`
//
#[proc_macro]
pub fn execute_cmd(_item: TokenStream) -> TokenStream {
    // Try it yourself!
    "\"execute a shell command at runtime...\"".parse().unwrap()
}
