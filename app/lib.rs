use futures::io::Close;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::HtmlInputElement;
use web_sys::{File, Blob, FormData};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use chrono::prelude::*;

use blob::Blob as add_blob;
use std::{str::FromStr, vec};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn start_websocket(file:File,file_name: String) -> Result<(), JsValue> {

    let ws1 = WebSocket::new("ws://192.168.248.131:8080")?;
    //let ws = WebSocket::open("ws://192.168.248.130:8080/ws1").unwrap();
    //let ws2 = WebSocket::open("ws://192.168.248.130:8080/").unwrap();
    // open the websocket connect
    let clone_ws1 =  ws1.clone();
    let file_name_clone = file_name.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        // get open time and log this
        let start_time = Utc::now().to_string();
        let message = format!("Websocket111 open success, {}!", start_time);
        console_log!("{}",message);
        
        let data = format!("name:abccc,file:dfa.txt");
        clone_ws1.send_with_str(&data).unwrap();
        
        // send file to server
        // match clone_ws.send_with_blob(&file){
        //     Ok(_) => {
        //         console_log!("File name successfully sent");
        //         // let window = web_sys::window().expect("no global `window` exists");
        //         // let document = window.document().expect("should have a document on window");
        //         // let body = document.body().expect("document should have a body");
        //         // let val = document.get_element_by_id("element_id").unwrap().dyn_into()::<HtmlInputElement>().unwrap().value();
        //     },
        //     Err(err) => console_log!("error sending message: {:?}",err),
        // }
        
        

        
    });
    ws1.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();


    let clone_ws1 =  ws1.clone();
    let file_clone = file.clone();
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            console_log!("message event, received arraybuffer: {:?}", abuf);
   
        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            console_log!("message event, received blob: {:?}", blob);

        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            if txt == "name" {
                console_log!("onmessage name");

                
                let file_size:f64 = file_clone.size();
        //let blob_num:f64 = file_size / 524_288_00.0;
        
        //let blob_num:f64 = (file_size / 500.0) + 1.0;

        //let blob_num:f64 = (file_size / 104_857_600.0) + 1.0;

        let blob_num:f64 = 2.0;
        let blob_num = blob_num as i64;

        if blob_num == 0 {
            clone_ws1.send_with_blob(&file_clone).unwrap();
        } else {
            //let blob_num = blob_num;
            //let my_blob: add_blob = add_blob::from_str("").unwrap();


            // for i in 0..blob_num{
                
            //     console_log!("that is slice {} time", i);
            //     // let start:i64 =i*500;
            //     // let end:i64 = start + 500;

            //     let start:i64 =i*104_857_600;
            //     let end:i64 = start + 104_857_600;

            //     // let start:i32 =i*500;
            //     // let end:i32 = start + 500;
            //     let start = start as f64;
            //     let end = end as f64;
            //     let slice = file.slice_with_f64_and_f64(start, end).unwrap();
            
               
                
            //     clone_ws.send_with_blob(&slice).unwrap();
            // }
            // console_log!("that is slice the last");
            // let blob_num = blob_num as f64; 
            // let slice = file.slice_with_f64_and_f64(blob_num * 104_857_600.0, file_size  ).unwrap();
            // // let slice = file.slice_with_f64_and_f64(blob_num * 500.0, file_size  ).unwrap();
            // clone_ws.send_with_blob(&slice).unwrap();


            let first_blob:f64 = file_size /2.0;
            let slice1 = file_clone.slice_with_f64_and_f64(0.0, first_blob).unwrap();
            
            // let  a= JsValue::from(slice1);
            // let arr_str = js_sys::Array::new();
            // arr_str.set(0,JsValue::from_str("user1"));
            // let blob_header = Blob::new_with_str_sequence(&arr_str).unwrap();

            // let b = JsValue::from(blob_header);
            // let arr_blob = js_sys::Array::new();
            // arr_blob.set(0,b);
            // arr_blob.set(1,a);
            // let chunk = Blob::new_with_blob_sequence(&arr_blob).unwrap();
            

            let s_chunk = (first_blob /3.0) as i64;

            for i in 0..3{
                console_log!("that is slice {} time", i);
                // let start:i64 =i*500;
                // let end:i64 = start + 500;

                let start:i64 =i*s_chunk;
                let end:i64 = start + s_chunk;

                // let start:i32 =i*500;
                // let end:i32 = start + 500;
                let start = start as f64;
                let end = end as f64;
                let slice = slice1.slice_with_f64_and_f64(start, end).unwrap();
            
               console_log!("{}",slice.size());
                
                clone_ws1.send_with_blob(&slice).unwrap();
            }

            
            
            

            // let slice_clone:add_blob = add_blob::new(slice1);
            // let my_blob: add_blob = add_blob::from_str("userabc1").unwrap();
            // add_blob::append_base64(&mut slice1, &my_blob).unwrap();
            //clone_ws1.send_with_blob(&chunk).unwrap();
        }
            }else{
                console_log!("message event, received Text: {:?}", txt);
            }
            
            
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }

        
    });
    // set message event handler on WebSocket
    ws1.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();



    let file_name_clone = file_name.clone();
    let onclose_callback = Closure::<dyn FnMut()>::new(move || {
        
        console_log!("Websocket close sueecess...{}", &file_name_clone);
    });
    ws1.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
    

    /*
    
    this is connect 2
    */
    let ws = WebSocket::new("ws://192.168.248.130:8080/")?;
    let cloned_ws = ws.clone();

    // set websocket data type to binary
    cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
    

    

    let clone_ws =  ws.clone();

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            console_log!("message event, received arraybuffer: {:?}", abuf);
   
        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            console_log!("message event, received blob: {:?}", blob);

        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            if txt == "name" {
                console_log!("onmessage name");

                let file_clone2 = file.clone();
                let file_size:f64 = file_clone2.size();
        //let blob_num:f64 = file_size / 524_288_00.0;
        
        //let blob_num:f64 = (file_size / 500.0) + 1.0;

        //let blob_num:f64 = (file_size / 104_857_600.0) + 1.0;

        let blob_num:f64 = 0.0;
        let blob_num = blob_num as i64;

        if blob_num == 0 {
            clone_ws.send_with_blob(&file_clone2).unwrap();
        } else {
            //let blob_num = blob_num;
            //let my_blob: add_blob = add_blob::from_str("").unwrap();


            // for i in 0..blob_num{
                
            //     console_log!("that is slice {} time", i);
            //     // let start:i64 =i*500;
            //     // let end:i64 = start + 500;

            //     let start:i64 =i*104_857_600;
            //     let end:i64 = start + 104_857_600;

            //     // let start:i32 =i*500;
            //     // let end:i32 = start + 500;
            //     let start = start as f64;
            //     let end = end as f64;
            //     let slice = file.slice_with_f64_and_f64(start, end).unwrap();
            
               
                
            //     clone_ws.send_with_blob(&slice).unwrap();
            // }
            // console_log!("that is slice the last");
            // let blob_num = blob_num as f64; 
            // let slice = file.slice_with_f64_and_f64(blob_num * 104_857_600.0, file_size  ).unwrap();
            // // let slice = file.slice_with_f64_and_f64(blob_num * 500.0, file_size  ).unwrap();
            // clone_ws.send_with_blob(&slice).unwrap();



            /*
            
                /2 send one blob
            */
            // let first_blob:f64 = file_size /2.0;
            
            // let slice2 = file.slice_with_f64_and_f64(first_blob, file_size).unwrap();
            

            // clone_ws.send_with_blob(&slice2).unwrap();



            let first_blob:f64 = file_size /2.0;
            let slice1 = file_clone2.slice_with_f64_and_f64(first_blob, first_blob).unwrap();
            
            
            // let  a= JsValue::from(slice1);
            // let arr_str = js_sys::Array::new();
            // arr_str.set(0,JsValue::from_str("user1"));
            // let blob_header = Blob::new_with_str_sequence(&arr_str).unwrap();

            // let b = JsValue::from(blob_header);
            // let arr_blob = js_sys::Array::new();
            // arr_blob.set(0,b);
            // arr_blob.set(1,a);
            // let chunk = Blob::new_with_blob_sequence(&arr_blob).unwrap();
            

            let s_chunk = (first_blob /3.0) as i64;
            let first_blob_i64 = first_blob as i64;
            for i in 0..3{
                console_log!("that is slice {} time", i);
                // let start:i64 =i*500;
                // let end:i64 = start + 500;

                let start:i64 =i*s_chunk + first_blob_i64;
                let end:i64 = start + s_chunk;

                // let start:i32 =i*500;
                // let end:i32 = start + 500;
                let start = start as f64;
                let end = end as f64;
                let slice = slice1.slice_with_f64_and_f64(start, end).unwrap();
                console_log!("{}start", start);
               console_log!("{}slice", slice.size());
               console_log!("{}slice1", slice1.size());
                clone_ws.send_with_blob(&slice).unwrap();
            }





            
        }
            }else{
                console_log!("message event, received Text: {:?}", txt);
            }
            
            
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }

        
    });
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    
    
    let file_name_clone = file_name.clone();

    
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        // get open time and log this
        let start_time = Utc::now().to_string();
        let message = format!("Websocket open success, {}!", start_time);
        console_log!("{}",message);
        
        
        // sent file name to server first
        match cloned_ws.send_with_str("name:abc,file:dfa.txt"){
            Ok(_) => console_log!("File name successfully sent"),
            Err(err) => console_log!("error sending message: {:?}",err),
        }
        
        
        // send file to server
        // match clone_ws.send_with_blob(&file){
        //     Ok(_) => {
        //         console_log!("File name successfully sent");
        //         // let window = web_sys::window().expect("no global `window` exists");
        //         // let document = window.document().expect("should have a document on window");
        //         // let body = document.body().expect("document should have a body");
        //         // let val = document.get_element_by_id("element_id").unwrap().dyn_into()::<HtmlInputElement>().unwrap().value();
        //     },
        //     Err(err) => console_log!("error sending message: {:?}",err),
        // }
        
        

        
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let file_name_clone = file_name.clone();
    let onclose_callback = Closure::<dyn FnMut()>::new(move || {
        
        console_log!("Websocket close sueecess...{}", &file_name_clone);
    });
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
    
    
    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str){
    alert(&format!("Hello, {}!",name));
}