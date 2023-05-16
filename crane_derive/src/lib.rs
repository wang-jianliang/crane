extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(AttrParser)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let struct_name = ast.ident;
    let fields = match ast.data {
        Data::Struct(data) => data.fields,
        _ => panic!("This macro can only be used for struct"),
    };

    let args_assign = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: #field_name
        }
    });

    let fields_extract = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            let #field_name = py_obj
                .get_item(stringify!(#field_name))
                .unwrap()
                .extract::<#field_type>()
                .unwrap();
        }
    });

    let parse_from_py_impl = quote! {
        impl AttrParser for #struct_name {
            fn from_py(py_obj: &PyAny) -> Self{
                #(#fields_extract)*
                #struct_name {
                    #(#args_assign),*
                }
            }
        }
    };
    // panic!("{}", parse_from_py_impl.to_string());
    parse_from_py_impl.into()
}
