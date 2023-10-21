use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Data, DeriveInput, Error, Field, Result, Token, Visibility};

pub fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;

    let attrs = attrs(input)?;
    let fields = fields(input)?;

    let mut filtered_fields: Vec<&Field> = Vec::with_capacity(fields.len());

    for field in fields.iter() {
        let field_attrs = field_attrs(field)?;

        if field_attrs.skip || attrs.skip_field(field) {
            continue;
        }

        filtered_fields.push(field);
    }

    let fields: Vec<_> = filtered_fields
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

struct Attrs {
    public_only: bool,
}

impl Attrs {
    fn skip_field(&self, field: &Field) -> bool {
        self.public_only && !matches!(field.vis, Visibility::Public(_))
    }
}

fn attrs(input: &DeriveInput) -> Result<Attrs> {
    let mut public_only = false;

    for attr in &input.attrs {
        if attr.path().is_ident("field_access") {
            attr.parse_nested_meta(|meta| {
                // #[field_access(public)]
                if meta.path.is_ident("public") {
                    public_only = true;
                    return Ok(());
                }

                Err(meta.error("unrecognized `field_access`"))
            })?;
        }
    }

    Ok(Attrs { public_only })
}

struct FieldAttrs {
    skip: bool,
}

fn field_attrs(field: &Field) -> Result<FieldAttrs> {
    let mut skip = false;

    for attr in &field.attrs {
        if attr.path().is_ident("field_access") {
            attr.parse_nested_meta(|meta| {
                // #[field_access(skip)]
                if meta.path.is_ident("skip") {
                    skip = true;
                    return Ok(());
                }

                Err(meta.error("unrecognized `field_access`"))
            })?;
        }
    }

    Ok(FieldAttrs { skip })
}
