use std::path::Path;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use rand::Rng;



const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(500000000);

const CLIENT_TIMEOUT: Duration = Duration::from_secs(10000000);

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use actix_web::web::Bytes;

use chrono::prelude::*;

use gloo_file::Blob;


pub struct MyWebSocket{
    hb: Instant,

}



impl MyWebSocket{
    pub fn new() -> Self {
        Self { 
            hb: Instant::now(),
            
        }
    }

    fn hb(&self, ctx:&mut <Self as Actor>::Context){
        ctx.run_interval(HEARTBEAT_INTERVAL, |act,ctx|{
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbear failed, disconnecting!");

                ctx.stop();

                return;
            }
            ctx.ping(b"");

        });
    }
}

impl Actor for MyWebSocket{
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        

        
    }


}


/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        //println!("WS: {msg:?}");

        //println!("OK");
        // let ws_data= &msg;
        // let ws_len=ws_data::Read
        // println!("WS: {ws_data:?}");

        // let mut ws_file = File::create("ws_data.txt").unwrap();
        // ws_file.write_all(msg.unwrap()).unwrap();

        // const ws_data :<Message> = msg.unwrap();
        // println!("{:?}", std::mem::size_of::<ws_data>());

        match msg {
                  
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                /*
                singal file version
                */
                // let file_name = text.to_string();
                // let mut file = File::create("file_name.txt").unwrap();
                // file.write_all(file_name.as_bytes()).unwrap();
                // ctx.text("name save success");

                
                // File::create(text.to_string()).expect("write failed");
                // ctx.text("name");

                
                let data = text.clone();
                let type_arr:Vec<&str> = data.split(",").collect();
                let mut path ="".to_owned();
                for i in &type_arr{
                let temp_str = &i[0..4];
                
                    match temp_str{
            
                        "name" => {
                            let a = &i[5..];
                            println!("{}",a);
                        
                            //path = a.to_string();
                        
                            path = format!("./{}/",a);

                            let path_clone = path.clone();
                            fs::create_dir_all(&path_clone).unwrap();
                            println!("{}",&path);
                            ctx.text("name")
                        }
            
                    
                        // "file" => {
                        //     let a = &i[5..];
                        //     println!("{}",a);
                        //     path = format!("{}{}",&path,a);
                        //     File::create(path).unwrap();
                        //     println!("{}",path)
                        //     //path = a.to_string();
                        //     //println!("{}",path);
                        // }
                        &_ => ctx.text(&*text)
                    }
                }
            

                


                // let event_header = &text[0..5];
                // let file_name = &text[5..];
                // if event_header == "name:"{
                    

                //     if Path::new("./1").exists(){
                //         fs::create_dir_all("./2").unwrap();
                //         let file_name_path = format!("./2/{}",&file_name);
                //         File::create(file_name_path).unwrap();
                //     }else{
                //         fs::create_dir_all("./1").unwrap();
                //         let file_name_path = format!("./1/{}",&file_name);
                //         File::create(file_name_path).unwrap();
                //     }
                //     ctx.text("name save success");
                //     ctx.text("name");

                // }else{
                //     ctx.text(text);
                // }
            },
            
            //{
                // dd if=/dev/zero of=static/10mb bs=1M count=10
            // let file = File::open("./static/10mb").unwrap();
            // let mut reader = BufReader::new(file);
            // let mut buffer = Vec::new();
        
            // reader.read_to_end(&mut buffer).unwrap();
            // ctx.binary(Bytes::from(buffer));
        //},
            Ok(ws::Message::Binary(bin)) => //ctx.binary(bin),
            {
                //if Path::new("./try.txt").exists() == false{
                //    let mut file = File::create("try.txt").unwrap();
                //    file.write_all(&bin).unwrap();
                //}else{

                //}

                /*
                singal file version
                */
                // //get file name
                // let mut file_name = String::new();
                // let mut name_txt = File::open("file_name.txt").expect("write failed");
                // name_txt.read_to_string(&mut file_name).expect("file_name.txt is empty");
                // let mut file = OpenOptions::new().append(true).open(&file_name).expect("write failed");
                // file.write_all(&bin).unwrap();

                // // record time for recive file
                // let udate = Utc::now().to_string();
                // let receive_message = format!("Receive file form firefox, {}!... {}", udate, &file_name);
                // ctx.text(receive_message);

                

                // seceond version
                // if Path::new("./file_name.txt").exists(){
                //     // get file name, get first file
                //     let mut file_name = String::new();
                //     let mut name_txt = File::open("file_name.txt").expect("write failed");
                //     name_txt.read_to_string(&mut file_name).expect("file_name.txt is empty");
                    
                //     // record time for recive file
                //     let udate = Utc::now().to_string();
                //     let receive_message = format!("Start to receive file form chrome, {}!... {}", udate, file_name);
                //     ctx.text(receive_message);
            
                //     // create new floder to save data
                //     // write data into file
                //     let mut file = OpenOptions::new().append(true).open(file_name).expect("write failed");
                //     file.write_all(&bin).unwrap();
                // }else{
                //     // this if first blob income
                //     // create new file_name.txt to save file name
                //     let mut file_name = String::new();
                //     let mut name_txt = File::open("file_name.txt").expect("write failed");
                //     name_txt.read_to_string(&mut file_name).expect("file_name.txt is empty");
                
                //     // create new floder to save data
                //     let mut file = File::create(file_name).unwrap();
                //     file.write_all(&bin).unwrap();

                //     // gen the time of the receive each blob save
                //     let end_time = Utc::now().to_string();
                //     let receive_message = format!("Receive success, {}!", end_time);
                //     ctx.text(receive_message);
                // }
                
                //ctx.binary(bin);
                println!("start");
                let udate = Utc::now().to_string();
                        let receive_message = format!("Start to receive file form chrome, {}!", udate);
                        println!("{}",receive_message);
                let mut rng = rand::thread_rng();

                let rnd_num = rng.gen_range(0..10000);
                let file_name = format!("{}.txt",rnd_num);

                let mut file = File::create(file_name).unwrap();
                        file.write_all(&bin).unwrap();
                        let udate = Utc::now().to_string();
                        let receive_message = format!("Start to receive file form chrome, {}!", udate);
                        println!("end{}",receive_message);
                        
            },
            
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },

            Ok(ws::Message::Continuation(blob)) => 
            {
                //let text = actix_http::ws::Item::Last(blob);
                //let blob_clown = blob::Last;
                //ctx.text(blob_clown.into());
                
                
                match blob {
                    actix_http::ws::Item::FirstText(a) => 
                    {
                        let mut file = File::create("firefox.zip").unwrap();
                        file.write_all(&a).unwrap();
                    },
                    actix_http::ws::Item::FirstBinary(b) => {
                        // get file name
                        let mut file_name = String::new();
                        let mut name_txt = File::open("file_name.txt").expect("write failed");
                        name_txt.read_to_string(&mut file_name).expect("file_name.txt is empty");
                        
                        // record time for recive file
                        let udate = Utc::now().to_string();
                        let receive_message = format!("Start to receive file form chrome, {}!... {}", udate, file_name);
                        ctx.text(receive_message);
                
                        //
                        let mut file = File::create(file_name).unwrap();
                        file.write_all(&b).unwrap();
                    },
                    actix_http::ws::Item::Continue(c) => {

                        // get file name
                        let mut file_name = String::new();
                        let mut name_txt = File::open("file_name.txt").expect("write failed");
                        name_txt.read_to_string(&mut file_name).expect("file_name.txt is empty");
                        
                        // record time for recive file
                        let udate = Utc::now().to_string();
                        let receive_message = format!("Start to receive file form chrome, {}!... {}", udate, file_name);
                        ctx.text(receive_message);

                        

                        // write data into file
                        let mut file = OpenOptions::new().append(true).open(file_name).expect("write failed");
                        file.write_all(&c).unwrap();
                    },
                    actix_http::ws::Item::Last(d) => {
                        // get file name
                        let mut file_name = String::new();
                        let mut name_txt = File::open("file_name.txt").expect("write failed");
                        name_txt.read_to_string(&mut file_name).expect("file_name.txt is empty");
                        
                        // record time for recive file
                        let udate = Utc::now().to_string();
                        let receive_message = format!("Start to receive file form chrome, {}!... {}", udate, file_name);
                        ctx.text(receive_message);



                        let mut file = OpenOptions::new().append(true).open(file_name).expect("write failed");
                        file.write_all(&d).unwrap();
                    },
                }
                
            },
            // {
            //     self.send_message("test");
            // }
            // {
            //     let mut file = File::create("669.pdf").unwrap();
            //     let file_data = blob.into(); 
            //     // let file_data_clown = a.into(file_data);
            //     ctx.text(file_data);
            // },
            _ => ctx.stop(),
        }
    }
}
