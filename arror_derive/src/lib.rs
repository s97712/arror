extern crate proc_macro;
use crate::proc_macro::TokenStream;

#[allow(pub_use_of_private_extern_crate)]
use syn::proc_macro2;

use quote::quote;
use syn;
use syn::*;
use syn::export::Span;

fn get_match_body<'a>(data: &Data, name: &Ident, type_map: &[&str], def_kind: &Ident, def_abort: bool)
  -> proc_macro2::TokenStream
{
  let match_list = match data {
    Data::Enum(enums) => {
      enums.variants.iter().filter_map(|variant| {
        let ver_id = &variant.ident;

        let abort = find_attr(&variant.attrs, ["abort"].as_ref()).is_some();
        let error_type = find_attr(&variant.attrs, type_map).or_else(|| {
          if abort {
            Some(def_kind.clone())
          } else {
            None
          }
        });

        error_type.map(|error_type_ident| {
          let args = match &variant.fields.iter().next() {
            Some(_) => {
              let args = variant.fields.iter().map(|_|Ident::new("_", Span::call_site()));
              quote! {
                (#(#args),*)
              }
            },
            None => quote!()
          };
          quote! {
            #name::#ver_id#args => ::arror::Arror::new (
              #abort,
              ::arror::ArrorKind::#error_type_ident,
              err
            ),
          }
        })
      })
    },
    _=> panic!("AppErr trait only support for enum.")
  };

  quote! {
    #(#match_list)*
    _ => ::arror::Arror::new (
      #def_abort,
      ::arror::ArrorKind::#def_kind,
      err
   )
  }
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
      fn from(err: #name) -> Arror {
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

