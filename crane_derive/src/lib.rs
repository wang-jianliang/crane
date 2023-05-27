extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(FromPyObject, attributes(from_py))]
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

    let fields_extract: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            if field.attrs.iter().any(|attr| attr.path.is_ident("from_py")) {
                let field_name = &field.ident;
                let field_type = &field.ty;
                Some(quote! {
                    obj.#field_name = py_obj
                        .get_item(stringify!(#field_name))?
                        .extract::<#field_type>()?;
                })
            } else {
                None
            }
        })
        .collect();

    let parse_from_py_impl = quote! {
        impl FromPyObject for #struct_name {
            fn from_py(py_obj: &PyAny) -> Result<Self, PyErr> {
                let mut obj = #struct_name::default();
                #(#fields_extract)*
                Ok(obj)
            }
        }
    };
    // panic!("{}", parse_from_py_impl.to_string());
    parse_from_py_impl.into()
}
