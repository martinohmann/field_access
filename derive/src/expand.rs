use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Data, DeriveInput, Error, Field, Result, Token};

pub fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;

    let fields: Vec<_> = fields(input)?
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().expect("field has a name");
            let name_str = name.to_string();
            (name, name_str)
        })
        .collect();

    let field_names = fields.iter().map(|(_, name_str)| quote!(#name_str));

    let immutable_arms = fields.iter().map(|(name, name_str)| {
        quote!(#name_str => {
            Some(&self.#name as &dyn ::core::any::Any)
        })
    });

    let mutable_arms = fields.iter().map(|(name, name_str)| {
        quote!(#name_str => {
            Some(&mut self.#name as &mut dyn ::core::any::Any)
        })
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::field_access::AnyFieldAccess for #name #ty_generics #where_clause {
            fn field_as_any(&self, field: &str) -> ::core::option::Option<&dyn ::core::any::Any> {
                match field {
                    #(#immutable_arms)*
                    _ => None
                }
            }

            fn field_as_any_mut(&mut self, field: &str) -> ::core::option::Option<&mut dyn ::core::any::Any> {
                match field {
                    #(#mutable_arms)*
                    _ => None
                }
            }

            fn field_names(&self) -> &'static [&'static str] {
                &[#(#field_names),*]
            }
        }
    })
}

type Fields = Punctuated<Field, Token![,]>;

fn fields(input: &DeriveInput) -> Result<&Fields> {
    use syn::Fields;

    let unsupported = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => return Ok(&fields.named),
            Fields::Unnamed(_) => "newtype structs",
            Fields::Unit => "unit structs",
        },
        Data::Enum(_) => "enums",
        Data::Union(_) => "unions",
    };

    Err(Error::new_spanned(
        input,
        format!("FieldAccess does not support {unsupported}"),
    ))
}
