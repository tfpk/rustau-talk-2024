use proc_macro::TokenStream;

#[proc_macro_derive(StructName1)]
pub fn sn1(item: TokenStream) -> TokenStream {
    let name_token = item.into_iter().nth(1).unwrap();
    let name = name_token.to_string();
    format!(
        "impl {name} {{ fn struct_name(&self) -> String {{ String::from(\"{name}\") }} }}"
    ).parse().unwrap()
}


use quote::quote;

#[proc_macro_derive(StructName2)]
pub fn sn2(item: TokenStream) -> TokenStream {
    let ast: syn::ItemStruct = syn::parse(item).unwrap();
    let name = &ast.ident;
    quote! {
        impl #name {
            fn struct_name(&self) -> String {
                format!("{}", stringify!(#name))
            }
        }
    }.into()
}
