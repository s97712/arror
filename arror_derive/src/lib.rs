extern crate proc_macro;
use crate::proc_macro::TokenStream;

#[allow(pub_use_of_private_extern_crate)]
use syn::proc_macro2;

use quote::quote;
use syn;
use syn::*;
use syn::export::Span;

fn get_match_body<'a>(data: &Data, name: &Ident, type_map: &[&str], def: &Ident, def_abort: bool)
  -> proc_macro2::TokenStream
{
  let match_list = match data {
    Data::Enum(enums) => {
      enums.variants.iter().filter_map(|variant| {
        let ver_id = &variant.ident;

        let abort = find_attr(&variant.attrs, ["abort"].as_ref()).is_some();
        let error_type = find_attr(&variant.attrs, type_map).or_else(|| {
          if abort {
            Some(def.clone())
          } else {
            None
          }
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
            #name::#ver_id#args => Arror::#error_type_ident(failure::Error::from(err), #abort),
          }
        })
      })
    },
    _=> panic!("AppErr trait only support for enum.")
  };

  quote! {
    #(#match_list)*
    _ => Arror::#def(failure::Error::from(err), #def_abort)
  }
}

fn get_error_ident(error_type: &str) -> Ident {
  Ident::new(error_type, Span::call_site())
}

fn find_attr<'a>(attrs: &'a Vec<Attribute>, attr_map: &[&str]) -> Option<Ident> {

  let res = attrs.iter().find(|attr| {
    match attr.path.get_ident() {
      Some(ident) => &ident.to_string() == "arror",
      None => false
    }
  }).and_then(|attr| {

    attr.parse_meta().ok().and_then(|meta| {
      match meta {
        Meta::List(list) => {
          list.nested.iter().find_map(|item| {
            match item {
              NestedMeta::Meta(word) => {
                match word.path().get_ident() {
                  Some(ident) => {
                    let name = ident.to_string();
                    attr_map.iter().find(|v| v == &&name).map(|_|ident.clone())
                  },
                  None => None
                }
                //true
              },
              _ => None
            }
          })
        },
        _ => None
      }
    })
  });
  res
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
  //let default_error = get_error_type(&ast.attrs).unwrap_or("User".to_owned());

  let type_map = ["User", "Internal", "Runtime", "Evil"];

  // let def_abort = find_attr(&ast.attrs, ["abort"].as_ref()).is_some();
  let default_error = Ident::new("User", Span::call_site());
  let default_error = find_attr(&ast.attrs, type_map.as_ref())
    .unwrap_or(default_error);

  let match_body = get_match_body(&ast.data, name, &type_map, &default_error, false);

  let gen = quote! {
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

#[proc_macro_derive(Arror, attributes(arror))]
pub fn app_err_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_app_err_macro(&ast)
}


