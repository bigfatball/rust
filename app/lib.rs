
use futures::io::Close;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::HtmlInputElement;
use web_sys::{File, Blob};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use chrono::prelude::*;

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

    // open the websocket connect
    let ws = WebSocket::new("ws://192.168.65.131:8080")?;
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
                let file_size:f64 = file.size();
        //let blob_num:f64 = file_size / 524_288_00.0;
        let blob_num:f64 = (file_size / 104_857_600.0) + 1.0;
        //let blob_num:f64 = (file_size / 500.0) + 1.0;
        let blob_num = blob_num as i64;

        if blob_num == 0 {
            clone_ws.send_with_blob(&file).unwrap();
        } else {
            let blob_num = blob_num;
            for i in 0..blob_num{
                
                console_log!("that is slice {} time", i);
                // let start:i64 =i*500;
                // let end:i64 = start + 500;

                let start:i64 =i*104_857_600;
                let end:i64 = start + 104_857_600;

                // let start:i32 =i*500;
                // let end:i32 = start + 500;
                let start = start as f64;
                let end = end as f64;
                let slice = file.slice_with_f64_and_f64(start, end).unwrap();
            
               
                
                clone_ws.send_with_blob(&slice).unwrap();
            }
            console_log!("that is slice the last");
            let blob_num = blob_num as f64; 
            let slice = file.slice_with_f64_and_f64(blob_num * 104_857_600.0, file_size  ).unwrap();
            // let slice = file.slice_with_f64_and_f64(blob_num * 500.0, file_size  ).unwrap();
            clone_ws.send_with_blob(&slice).unwrap();
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
        match cloned_ws.send_with_str(&file_name_clone){
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