extern crate proc_macro;

use std::{fs, string};
use proc_macro::{TokenStream, Span};
use serde_json::{json, Value};
use syn::{parse_macro_input, DeriveInput, Data, Ident};
use quote::{quote};
use handlebars::Handlebars;
use rand::distributions::{Alphanumeric, DistString};

#[proc_macro]
pub fn route(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::LitStr);

    let str_val = item.value();

    let out = quote!(
        static route: &str = #str_val;
    );

    out.into()
}

#[proc_macro_derive(Page, attributes(updateAble, system))]
pub fn page(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    
    let name = input.ident;

    let Data::Struct(data) = input.data else {
        unimplemented!("non structs");
    };
    let random_ids: Vec<String> = data.fields.iter()
        .filter(|field| field.attrs.iter().all(|attr| !attr.path.is_ident("system")))
        .map(|field| {
        let Some(ident) = &field.ident else {
            unimplemented!("tuples");
        };
        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 30);
        return random_string;
    }).collect();
    let json_able = data.fields.iter()
        //dont serialise system fields
        .filter(|field| field.attrs.iter().all(|attr| !attr.path.is_ident("system")))
        .enumerate()
        .map(|(index, field)| {

        let Some(ident) = &field.ident else {
            unimplemented!("tuples");
        };
        let rndo = random_ids.get(index).expect("in macro expansion invalid id");
        quote!(
            let buff = buff.replace(&format!("{{{}}}", stringify!(#ident)), 
                &format!("<span id={}>{}</span>", #rndo, self.#ident)
            );
            //stringify!(#ident) : format!("<span id={}>{}</span>", #str, self.#ident)
        )
    });
    
    let field_data = data.fields.iter()
        .filter(|field| field.attrs.iter().all(|attr| !attr.path.is_ident("system")))
        .enumerate()
        //remove all fields that arent registered as updateAble
        //these fields will get updated on stateHasChanged() but not dynamically
        .filter(|(index,field)| 
            field.attrs.iter().any(|attr| attr.path.is_ident("updateAble"))
        ).map(|(index,field)| 
            (index, field.ty.clone(), field.ident.clone().unwrap())
        )
        //function returns (datatype, field_name, set_field_name) for every updateable field
        .map(|(index, typ, ident)| 
            (index, typ,
                ident.clone(), 
                Ident::new(
                    &format!("set_{}",ident), 
                    Span::call_site().into()
                )
            )
        );

    //get a static hashmap for every updateable field with "field_name" => set_field()
    let function_map = field_data.clone()
        .map(|(index, _typ, _name, func_name)| (func_name.clone(), format!("{}", func_name)))
        .map(|(func_name, func_str)| {
            return quote!(
                #func_str => #name::#func_name,
            );
        });
    //create the functions that update the field
    let helpers = field_data.clone()
        .map(|(index, typ, name, funcName)| {
            let id = random_ids.get(index).expect("error fetching random");
            return quote!(
                fn #funcName(&mut self, new_val: &str){
                    let new_val: #typ = new_val.parse().expect("parse_error");
                    self.#name = new_val.clone();
                    if let Some(s) = &self.sender {
                        s.send(format!("{}:{}", #id, new_val));
                    }
                }
            );
        });


    

    let r = quote!(
        impl Page for #name {
            fn set_tx(&mut self, val: Sender<String>) {
                self.sender = Some(val);

                self.set_name("test_adapt");
            }

            fn to_json(&self) -> Value {
                let buff = PAGE;
                //#( #json_able)*
                /*serde_json::json!({
                    #( #json_able),*
                })*/
                serde_json::json!({})
            }
            fn eval(&self) -> String {
                let page = fs::read_to_string("./webServer/src/pages/Default.html").unwrap();
                
                //let page_content = Handlebars::new()
                    //.render_template(PAGE, &self.to_json()).unwrap();
                let buff = PAGE;
                #( #json_able)*
                page.replace("{{body}}", &buff)
            }
            fn state_has_changed(&self){
                println!("should update now lol");
            }
        }
        static FUNCS: phf::Map<&'static str, fn(&mut #name, &str) -> ()> = phf::phf_map!{
            #( #function_map )*
          };
        impl #name {
            #( #helpers)*
        }

    );
    TokenStream::from(r)
}