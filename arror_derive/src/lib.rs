extern crate proc_macro;
use crate::proc_macro::TokenStream;

#[allow(pub_use_of_private_extern_crate)]
use syn::proc_macro2;

use quote::quote;
use syn;
use syn::*;
use syn::export::Span;

fn get_match_body(data: &Data, name: &Ident, def: &str) -> proc_macro2::TokenStream {
  let match_list = match data {
    Data::Enum(enums) => {
      enums.variants.iter().filter_map(|variant| {
        let ver_id = &variant.ident;
        let error_type = get_error_type(&variant.attrs).map(|t| {
          get_error_ident(&t)
        });

        error_type.map(|error_type_ident| {
          let args = match &variant.fields.iter().next() {
            Some(_) => {
              let args = variant.fields.iter().map(|_|Ident::new("_", Span::call_site()));
              quote!{
              (#(#args),*)
            }
            },
            None => quote!()
          };
          quote! {
            #name::#ver_id#args => Arror::#error_type_ident(From::from(err)),
          }
        })
      })
    },
    _=> panic!("AppErr trait only support for enum.")
  };

  let def_error_ident = get_error_ident(def);
  quote! {
    #(#match_list)*
    _ => Arror::#def_error_ident(From::from(err))
  }
}

fn get_error_ident(error_type: &str) -> Ident {
  Ident::new(error_type, Span::call_site())
}

fn get_error_type(attrs: &Vec<Attribute>) -> Option<String> {

  let error_type = attrs.iter().find(|attr| {
    match attr.path.get_ident() {
      Some(ident) => &ident.to_string() == "arror",
      None => false
    }
  }).and_then(|attr| {
    attr.parse_meta().ok().and_then(|meta| {
      match meta {
        Meta::List(list) => {
          list.nested.iter().next().and_then(|item| {
            match item {
              NestedMeta::Meta(word) => {
                word.path().get_ident().map(|name| name.to_string())
              },
              NestedMeta::Lit(lit) => {
                match lit {
                  Lit::Str(name) => Some(name.value()),
                  _ => None
                }
              }
            }
          })
        },
        _ => None
      }
    })
  });

  error_type
}

fn impl_app_err_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let default_error = get_error_type(&ast.attrs).unwrap_or("User".to_owned());

  let match_body = get_match_body(&ast.data, name, &default_error);

  let gen = quote! {
    use std::convert::From;
    impl From<#name> for Arror {
      fn from(err: #name) -> Self {
        match &err {
          #match_body
        }
      }
    }
  };
  gen.into()
}

#[proc_macro_derive(Arror, attributes(user, internal, runtime, evil, arror))]
pub fn app_err_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_app_err_macro(&ast)
}


