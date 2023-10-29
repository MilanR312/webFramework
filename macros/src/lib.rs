#![feature(proc_macro_span)]

extern crate proc_macro;

mod loading;



use std::{fs, path::PathBuf, f32::consts::E, collections::HashMap};
use loading::{load_pages_internal, load_components_internal};
use proc_macro::{TokenStream, Span};

use syn::{parse_macro_input, DeriveInput, Data, Ident, ItemFn};
use quote::{quote, ToTokens};
use rand::distributions::{Alphanumeric, DistString};


#[proc_macro]
pub fn load_pages(item: TokenStream) -> TokenStream{
    load_pages_internal(item)
}
#[proc_macro]
pub fn load_components(item: TokenStream) -> TokenStream {
    load_components_internal(item)
}

#[proc_macro]
pub fn route(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::LitStr);

    let str_val = item.value();

    let out = quote!(
        pub static ROUTE: &str = #str_val;
    );

    out.into()
}
#[proc_macro]
pub fn html_from_file(item: TokenStream) -> TokenStream{
    let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 30);

    let data = parse_macro_input!(item as syn::LitStr);
    let cont = fs::read_to_string(data.value()).expect("file not found");
    let o = quote!(
        use achernar::html::HtmlRoot;
        use achernar::page::Html;

        //lazy static depricated
        /* use lazy_static::lazy_static;
        lazy_static!{

            pub static ref HTML: Html = Html{
                content: HtmlRoot::from_str(&#cont),
                id: #id.to_string()
            };
        }*/
        use once_cell::sync::Lazy;
        pub static HTML: Lazy<Html> = Lazy::new(|| Html{
            content: HtmlRoot::from_str(&#cont),
            id: #id.to_string()
        });
        //pub static HTML: &str = #out;
    );
    //let _ = fs::write("log", o.to_string());
    o.into()
}

#[proc_macro]
pub fn html(item: TokenStream) -> TokenStream {
    let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 30);

    let out = item.to_string();
    let out = out.replace("< ", "<");
    let out = out.replace(" >", ">");
    let out = out.replace("/ ", "/");
    
    //remove spaces
    let r = regex::Regex::new("\\| (\\w+) \\|").unwrap();
    let out = r.replace_all(&out, "|${1}|");

    let out = out.to_string();

    //get all functions declared inside html
    let r = regex::Regex::new("(\\w+) = \\|(\\w+)\\|").unwrap();
    let out = r.replace_all(&out, "${1}=\"(() => {
        let loc = window.location.pathname;
        conn.send(loc + `,${2},0`);
        })();\"");


    println!("content =========  {}", out);
    let o = quote!(
        use achernar::html::*;
        use achernar::page::Html;
        use once_cell::sync::Lazy;
        pub static HTML: Lazy<Html> = Lazy::new(|| Html{
                    content: HtmlRoot::from_str(&#out),
                    id: #id.to_string()
                });
        //pub static HTML: &str = #out;
    );
    //let _ = fs::write("log", o.to_string());
    o.into()

}

#[proc_macro]
pub fn code(item: TokenStream) -> TokenStream {
    let data = parse_macro_input!(item as syn::Block);

    let data = data.stmts;
    //get all the functions defined in the statement
    let funcs = data.clone().into_iter()
        .filter(|stmt| matches!(stmt, syn::Stmt::Item(_x)))
        .map(|stmt| {
            match stmt {
                syn::Stmt::Item(x) => x,
                _ => unreachable!()
            }
        }).map(|it| {
            match it {
                syn::Item::Fn(x) => x,
                _ => unreachable!(),
            }
        });

    //get the init function if it exists
    let init_func = funcs.clone()
        .filter(|func| func.sig.ident == "init");

    //call the init function if exists
    let init_call = init_func.clone()
        .map(|func| {
            let name = func.sig.ident;
            quote!(
                Self::#name(&mut out);
            )
        })
        .nth(0);

    let init_func = init_func
        .map(|f| f.to_token_stream());

    let funcs = funcs
        //filter out async functionsm TODO: warn here?
        .filter(|func| func.sig.asyncness.is_some())
        .map(|f| f.to_token_stream());

    let variables = data.clone().into_iter()
        .filter(|stmt| !matches!(stmt, syn::Stmt::Item(_x)))
        .map(|stmt| {
            match stmt {
                syn::Stmt::Semi(expr,_delim) => expr.to_token_stream(),
                syn::Stmt::Expr(e) => e.to_token_stream(),
                syn::Stmt::Local(e) => e.to_token_stream(),
                _ => unreachable!()
            }
        });

    //let _f: TokenStream = funcs.clone().next().expect("no function found").into();
    
    let func_map = funcs.clone()
        //.map(|func| func.into())
        .map(|func| {
            let x = syn::parse2::<ItemFn>(func).unwrap();
            let id = x.sig.ident;
            let inp = x.sig.inputs;
            let mut inp = inp.iter();
            let _a = inp.next().is_some();
            let typ = match *inp.next().unwrap() {
                syn::FnArg::Typed(ref x) => &x.ty,
                syn::FnArg::Receiver(ref x) => unimplemented!(),
            };
            let typ = *typ.clone();
            //let _b = inp.next().is_some();
            let s = id.to_string();
            println!("found func {s} with type {typ:?}");
            quote!(
                out.insert(
                    #s,
                    Box::new(|a, b| {
                        //b is the argument
                        info!("trying to parse {b}");
                        let value: Result<_,_> = serde_json::from_str(b);
                        if let Ok(val) = value {
                            info!("parse succesfull {b}");
                            Box::pin(Self::#id(a, val))
                        } else {
                            info!("parse failed");
                            async fn nop(){}
                            Box::pin(nop())
                        }
                }));
            )
        });
    let span = Span::call_site();
    let source = span.source_file().path();
    let source = source.file_name().unwrap();
    let file_name = source.to_str().unwrap();
    let file_name = file_name.split(".").next().unwrap();

    println!("---------------------------------------- source = {:?}", file_name);
    let struct_name = {
        let mut c = file_name.chars();
        match c.next() {
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            None => String::new(),
        }
    };
    println!("---------------------------------------- source = {:?}", struct_name);

    let str = struct_name.clone();

    let struct_name = Ident::new(
        &struct_name,
        Span::call_site().into()
    );    
    let vars_copy  = data.clone().into_iter()
        .filter(|stmt| !matches!(stmt, syn::Stmt::Item(_x)))
        .map(|stmt| {
            match stmt {
                syn::Stmt::Semi(expr,_delim) => expr,
                _ => unreachable!()
            }
        }).map(|field| {
            match field {
                syn::Expr::Type(f) => f,
                e => panic!("non field or function item in code block {}", e.to_token_stream()),
            }
        }).map(|field| {
            let mut field = field.clone();
            field.attrs.clear();
            let ty = *field.ty;
            let id = *field.expr;
            quote!(
                #id: <#ty>::default()
            )
        });

    let this_file = Span::call_site().source_file();
    //let path = PathBuf::from(this_file);
    //let dir = path.parent().unwrap();
    let parent_file = this_file.path();
    let parent_file = parent_file.parent().unwrap();
    let parent_file = parent_file.components().last().unwrap();

    let parent_file = parent_file.as_os_str().to_str().unwrap();
    let to_derive = if parent_file == "pages" {
        quote!(Page)
    } else {
        quote!(Component)
    };
    
    let out = quote!(
        use std::sync::Arc;

        use achernar::page::*;
        use futures_util::{SinkExt, stream::SplitSink};

        use tokio::{net::TcpStream, sync::{Mutex, MutexGuard}};

        use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
        use std::collections::HashMap;
        use tracing::info;

        #[derive(#to_derive)]
        pub struct #struct_name {
            #[system]
            sender: Option<Arc<Mutex<SplitSink<WebSocketStream<TcpStream>,Message>>>>,
    
            #( #variables ,)*
        }

        impl #struct_name {
           #( #funcs )*
           
          #( #init_func )*
        }
        use achernar::page::*;
        /*use async_trait::async_trait;*/
        #[async_trait]
        impl UserFunctions for #struct_name {
            async fn user_execute(&mut self, method: &str, arg: &str){
                info!("searching for user methods");
                let methods = {
                    let mut out: HashMap<&'static str, Handler<Self>> = HashMap::new();
                    #( #func_map )*
                    out
                };
                if let Some(func) = methods.get(method){
                    info!("executing user method");
                    (func)(self, arg).await;
                    return;
                }
            }
        }

        impl #struct_name {
            pub fn new() -> Self{
                let mut out = Self {
                    sender: None,
                    #( #vars_copy ,)*
                };
                #init_call
                out
            }
        }
    );
    //let _ = fs::write(format!("{}.log.rs", str), out.to_string());
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
        let Some(_ident) = &field.ident else {
            unimplemented!("tuples");
        };
        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 30);
        return random_string;
    }).collect();
    let update_able = data.fields.iter()
        //dont serialise system fields
        .filter(|field| field.attrs.iter().all(|attr| !attr.path.is_ident("system")))
        .enumerate()
        .map(|(index, field)| {

        let Some(ident) = &field.ident else {
            unimplemented!("tuples");
        };
        let rndo = random_ids.get(index).expect("in macro expansion invalid id");
        quote!(
            let buff = buff.replace(&format!("|{}|", stringify!(#ident)), 
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
        .filter(|(_index,field)| 
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
        .map(|(_index, _typ, _name, func_name)| (func_name.clone(), format!("{}", func_name)))
        .map(|(func_name, func_str)| {
            return quote!(
                //#func_str => AsyncFnPtr::<#name>::new(#name::#func_name)
                //TODO 
                //#func_str => #name::#func_name
                //#func_str => Box::new(#name::#func_name)
                //#func_str => Box::new(move |a,b| Box::pin(#name::#func_name(a,b)))
                out.insert(#func_str, 
                    Box::new(|a, b| {
                        let value: Result<_,_> = serde_json::from_str(b);
                        if let Ok(val) = value {
                            Box::pin(Self::#func_name(a, val))
                        } else {
                            async fn nop(){}
                            Box::pin(nop())
                        }
                    }));
                   // Box::new(#name::#func_name))
            );
        });
    //create the functions that update the field
    let helpers = field_data.clone()
        .map(|(index, typ, name, func_name)| {
            let id = random_ids.get(index).expect("error fetching random");
            return quote!(
                //TODO: edited new val to typ here but fix dyn dispatch without all str?
                async fn #func_name(&mut self, new_val: #typ){
                    info!("in setter");
                    /*let Ok(new_val) = new_val.parse::<#typ>() else {
                        if let Some(s) = &self.sender {
                            let mut send = s.lock().await;
                            let _ = send.send(
                                tokio_tungstenite::tungstenite::Message::Text(
                                    format!("value \"{}\" is not a valid type of '{}'", new_val, stringify!(#typ))
                                    )
                            ).await;
                            let _ = send.flush().await;

                        }
                        return;
                    };*/
                    info!("parse succesfull");
                    self.#name = new_val.clone();
                    //info!("checking if sender exists");
                    if let Some(s) = &self.sender {
                        let mut send = s.lock().await;
                        let _ = send.send(
                            tokio_tungstenite::tungstenite::Message::Text(
                            format!("{}:{}", #id, new_val)
                            )
                        ).await;
                        info!("flushing");
                        let _ = send.flush().await;

                    }
                }
            );
        });
   

    let r = quote!(
        use async_trait::async_trait;
        #[async_trait]
        impl Page for #name {

            fn set_sender(&mut self, sender: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>,Message>>>){
                self.sender = Some(sender);
            }
            fn eval(&self, components: MutexGuard<'_, HashMap<String, Box<dyn Component + Sync + Send>>>) -> String {
                use std::fs;
                //let page = fs::read_to_string("./webServer/src/pages/Default.html").unwrap();
                
                //let page_content = Handlebars::new()
                    //.render_template(PAGE, &self.to_json()).unwrap();
                let mut x = HTML.content.clone();
                info!("components = ");
                components.iter().for_each(|(name, _)| info!("{name}") );
                components.iter().for_each(|(name, comp)|{
                    x.replace_component(name, &comp.eval());
                });
                let buff = format!("<div id={}>{}</div>", HTML.id, x);
                // HTML.content;
                #( #update_able)*
                //page.replace("{{body}}", &buff)
                buff
            }
            fn state_has_changed(&self){
                println!("should update now lol");
            }


            async fn execute(&mut self, method: &str, arg: &str){
                let methods = {
                    let mut out: HashMap<&'static str, Handler<Self>> = HashMap::new();
                    #( #function_map )*
                    out
                };
                info!(?method, "executing setter");
                if let Some(func) = methods.get(method){
                    info!("found system method");
                    (func)(self, arg).await;
                } else {
                    info!("didnt find system method");
                    self.user_execute(method, arg).await;
                }
            }
            /*fn get_methods(&self) -> HashMap<&'static str, Handler<dyn Page>>{
                let mut out: HashMap<&'static str, Handler<dyn Page>> = HashMap::new();
                #( #function_map )*
                //out.insert("set_val", Box::new(|a, b| Box::pin(Self::set_val(a, b))));
                out
            }*/

        }


        impl #name {
            #( #helpers)*
        }

    );
    
    TokenStream::from(r)
}

#[proc_macro_derive(Component, attributes(updateAble, system))]
pub fn component(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    
    let name = input.ident;

    let Data::Struct(data) = input.data else {
        unimplemented!("non structs");
    };
    let random_ids: Vec<String> = data.fields.iter()
        .filter(|field| field.attrs.iter().all(|attr| !attr.path.is_ident("system")))
        .map(|field| {
        let Some(_ident) = &field.ident else {
            unimplemented!("tuples");
        };
        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 30);
        return random_string;
    }).collect();
    let update_able = data.fields.iter()
        //dont serialise system fields
        .filter(|field| field.attrs.iter().all(|attr| !attr.path.is_ident("system")))
        .enumerate()
        .map(|(index, field)| {

        let Some(ident) = &field.ident else {
            unimplemented!("tuples");
        };
        let rndo = random_ids.get(index).expect("in macro expansion invalid id");
        quote!(
            let buff = buff.replace(&format!("|{}|", stringify!(#ident)), 
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
        .filter(|(_index,field)| 
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
        .map(|(_index, _typ, _name, func_name)| (func_name.clone(), format!("{}", func_name)))
        .map(|(func_name, func_str)| {
            return quote!(
                //#func_str => AsyncFnPtr::<#name>::new(#name::#func_name)
                //TODO 
                //#func_str => #name::#func_name
                //#func_str => Box::new(#name::#func_name)
                //#func_str => Box::new(move |a,b| Box::pin(#name::#func_name(a,b)))
                out.insert(#func_str, 
                    Box::new(|a, b| Box::pin(Self::#func_name(a, b))));
                   // Box::new(#name::#func_name))
            );
        });
    //create the functions that update the field
    let helpers = field_data.clone()
        .map(|(index, typ, name, func_name)| {
            let id = random_ids.get(index).expect("error fetching random");
            return quote!(
                async fn #func_name(&mut self, new_val: &str){
                    info!("in setter");
                    let Ok(new_val) = new_val.parse::<#typ>() else {
                        if let Some(s) = &self.sender {
                            let mut send = s.lock().await;
                            let _ = send.send(
                                tokio_tungstenite::tungstenite::Message::Text(
                                    format!("value \"{}\" is not a valid type of '{}'", new_val, stringify!(#typ))
                                    )
                            ).await;
                            let _ = send.flush().await;

                        }
                        return;
                    };
                    info!("parse succesfull");
                    self.#name = new_val.clone();
                    //info!("checking if sender exists");
                    if let Some(s) = &self.sender {
                        let mut send = s.lock().await;
                        let _ = send.send(
                            tokio_tungstenite::tungstenite::Message::Text(
                            format!("{}:{}", #id, new_val)
                            )
                        ).await;
                        info!("flushing");
                        let _ = send.flush().await;

                    }
                }
            );
        });


   

    let r = quote!(
        use async_trait::async_trait;
        #[async_trait]
        impl Component for #name {

            /*fn set_sender(&mut self, sender: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>,Message>>>){
                self.sender = Some(sender);
            }*/
            fn eval(&self) -> String {
                use std::fs;
                //let page = fs::read_to_string("./webServer/src/pages/Default.html").unwrap();
                
                //let page_content = Handlebars::new()
                    //.render_template(PAGE, &self.to_json()).unwrap();
                let buff = format!("<div id={}>{}</div>", HTML.id, HTML.content);
                
                
               // HTML.content;
                #( #update_able)*
                //page.replace("{{body}}", &buff)
                buff
            }
            /*fn state_has_changed(&self){
                println!("should update now lol");
            }*/


            async fn execute(&mut self, method: &str, arg: &str){
                let methods = {
                    let mut out: HashMap<&'static str, Handler<Self>> = HashMap::new();
                    #( #function_map )*
                    out
                };
                info!(?method, "executing setter");
                if let Some(func) = methods.get(method){
                    info!("found system method");
                    (func)(self, arg).await;
                } else {
                    info!("didnt find system method");
                    self.user_execute(method, arg).await;
                }
            }
            /*fn get_methods(&self) -> HashMap<&'static str, Handler<dyn Page>>{
                let mut out: HashMap<&'static str, Handler<dyn Page>> = HashMap::new();
                #( #function_map )*
                //out.insert("set_val", Box::new(|a, b| Box::pin(Self::set_val(a, b))));
                out
            }*/

        }


        impl #name {
            #( #helpers)*
        }

    );
    
    TokenStream::from(r)
}
