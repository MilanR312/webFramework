use std::fs;
use proc_macro::{TokenStream, Span};

use syn::{parse_macro_input, DeriveInput, Data, Ident, ItemFn};
use quote::{quote, ToTokens};
use rand::distributions::{Alphanumeric, DistString};


pub fn load_pages_internal(item: TokenStream) -> TokenStream {
    let data = parse_macro_input!(item as syn::LitStr);

    let _path = data.clone();

    let path_val = data.value();
    let module = path_val.split("/")
        .last()
        .expect("no path added to end");


//./webServer/src/
    let read_dir = fs::read_dir(data.value());
    let read_dir = match read_dir {
        Ok(x) => x,
        Err(e) => unimplemented!("pages directory must exist '{}' current dir: {:?} || {:?}", data.value(), std::env::current_dir(), e),
    };

    let all_files = read_dir
        .map(|file| {
            let Ok(file) = file else {
                unimplemented!("idk what would cause this");
            };
            let file_type = file.file_type();
            let Ok(file_type) = file_type else {
                unimplemented!("filetype unable to be read");
            };
            (file, file_type)
        })
        .filter(|(_file, file_type)| file_type.is_file())
        //exlude mod file
        .filter(|(file, _file_type)| file.file_name() != "mod.rs" )
        //exclude non rust files
        .filter(|(file, _file_type)| file.file_name().to_str().unwrap().ends_with(".rs") )
        .map(|(file, _file_type)|  {
            let file_name = file.file_name().to_str().expect("err").to_owned(); 

            let file_name = file_name.split(".").next().unwrap();

            let struct_name = {
                let mut c = file_name.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            };
            let file_name = Ident::new(
                file_name,
                Span::call_site().into()
            );
            let struct_name = Ident::new(
                &struct_name,
                Span::call_site().into()
            );
            let module_name = Ident::new(
                &module,
                Span::call_site().into()
            );
            quote!(
                out.insert( 
                    #module_name::#file_name::ROUTE.to_string(), 
                    //Box::new(#struct_name)
                    Box::new(#module_name::#file_name::#struct_name::new()) 
                );
            )
            //quote!( 
            //    #file_name => Box::new(shared::#file_name::#struct_name::new()),
            //)
        });
    
    
    let out = quote!(
        {
            use std::collections::HashMap;
            use std::boxed;
            use std::sync::Arc;
            use tokio::sync::Mutex;
            let mut output: Arc<Mutex<HashMap<String, Box<dyn Page + Sync + Send>>>> = Arc::new(Mutex::new(HashMap::new()));
            {
                let mut out = output.lock().await;
                #( #all_files )*
            }
            output
        }
        
        //std::collections::HashMap::from([ #( #all_files),*  ]);
        /*let shared: phf::Map<&'static str, Box<dyn Page>> = phf::phf_map!{ 
            #( #all_files )* 
        };*/
    );
    out.into()

}

pub fn load_components_internal(item: TokenStream) -> TokenStream {
    let data = parse_macro_input!(item as syn::LitStr);

    let _path = data.clone();

    let path_val = data.value();
    let module = path_val.split("/")
        .last()
        .expect("no path added to end");

    let read_dir = fs::read_dir(data.value());
    let read_dir = match read_dir {
        Ok(x) => x,
        Err(e) => unimplemented!("shared directory must exist '{}' current dir: {:?} || {:?}", data.value(), std::env::current_dir(), e),
    };


    let all_files = read_dir
        .map(|file| {
            let Ok(file) = file else {
                unimplemented!("idk what would cause this");
            };
            let file_type = file.file_type();
            let Ok(file_type) = file_type else {
                unimplemented!("filetype unable to be read");
            };
            (file, file_type)
        })
        .filter(|(_file, file_type)| file_type.is_file())
        //exlude mod file
        .filter(|(file, _file_type)| file.file_name() != "mod.rs" )
        //exclude non rust files
        .filter(|(file, _file_type)| file.file_name().to_str().unwrap().ends_with(".rs") )
        .map(|(file, _file_type)|  {
            let file_name = file.file_name().to_str().expect("err").to_owned(); 

            let file_name = file_name.split(".").next().unwrap();

            let struct_name = {
                let mut c = file_name.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            };
            let old_file = file_name.to_owned();
            let file_name = Ident::new(
                file_name,
                Span::call_site().into()
            );
            let struct_name = Ident::new(
                &struct_name,
                Span::call_site().into()
            );
            let module_name = Ident::new(
                &module,
                Span::call_site().into()
            );
            quote!(
                out.insert( 
                    //as opposed to pages we save components by their filename instead of the route
                    #old_file.to_string(), 
                    //Box::new(#struct_name)
                    Box::new(#module_name::#file_name::#struct_name::new()) 
                );
            )
            //quote!( 
            //    #file_name => Box::new(shared::#file_name::#struct_name::new()),
            //)
        });
        let out = quote!(
            {
                use std::collections::HashMap;
                use std::boxed;
                use std::sync::Arc;
                use tokio::sync::Mutex;
                let mut output: Arc<Mutex<HashMap<String, Box<dyn Component + Sync + Send>>>> = Arc::new(Mutex::new(HashMap::new()));
                {
                    let mut out = output.lock().await;
                    #( #all_files )*
                }
                output
            }
            
            //std::collections::HashMap::from([ #( #all_files),*  ]);
            /*let shared: phf::Map<&'static str, Box<dyn Page>> = phf::phf_map!{ 
                #( #all_files )* 
            };*/
        );
        out.into()
}