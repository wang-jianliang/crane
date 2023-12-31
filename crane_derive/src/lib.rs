extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Type};

#[proc_macro_derive(FromPyObject, attributes(from_py))]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let struct_name = ast.ident;
    let fields = match ast.data {
        Data::Struct(data) => data.fields,
        _ => panic!("This macro can only be used for struct"),
    };

    let fields_extract: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            if field.attrs.iter().any(|attr| attr.path.is_ident("from_py")) {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let err_branch = match field_type {
                    Type::Path(type_path) if type_path.path.segments.last().map_or(false, |seg|seg.ident == "Option") => {
                        quote! {
                            Err(err) => None
                        }
                    },
                    _ => quote! {
                        Err(err) => return Err(Error::new("Required field ".to_owned() + stringify!(#field_name) + " does not exist"))
                    }
                };
                Some(quote! {
                    obj.#field_name = match py_obj.get_item(stringify!(#field_name), vm) {
                        Ok(item) => match item.try_into_value::<#field_type>(vm) {
                            Ok(value) => value,
                            Err(err) => {
                                return Err(Error::new("Invalid value type of field ".to_owned() + stringify!(#field_name)));
                            }
                        },
                        #err_branch,
                    };
                })
            } else {
                None
            }
        })
        .collect();

    let parse_from_py_impl = quote! {
        impl FromPyObject for #struct_name {
            fn from_py(py_obj: &PyObjectRef, vm: &VirtualMachine) -> Result<Self, Error> {
                let mut obj = #struct_name::default();
                #(#fields_extract)*
                Ok(obj)
            }
        }
    };
    // panic!("{}", parse_from_py_impl.to_string());
    parse_from_py_impl.into()
}
