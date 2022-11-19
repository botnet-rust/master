extern crate websocket;

use websocket::{sync::Server, OwnedMessage};
use std::str::FromStr;
use std::thread;
use std::process;


fn main() {
	let server = Server::bind("127.0.0.1:1234").unwrap();


	for request in server.filter_map(Result::ok) {
        thread::spawn(|| {

            let client = request.accept().unwrap();
            let ip = client.peer_addr().unwrap();
            let (mut reciever, mut sender) = client.split().unwrap();

            macro_rules! log {
                ($tag:expr) => {println!("({}) [{}]",ip ,$tag)};
                ($tag:expr,$text:expr) => {println!("({}) [{}] {}",ip ,$tag,$text)}
            }

            macro_rules! send {

                ($msg:expr) => {

                    sender.send_message(&$msg).unwrap();

                }
            }

            macro_rules! sendText {

                ($msg:expr) => {

                    sender.send_message(&OwnedMessage::Text(format!("{}",$msg))).unwrap();

                };

                ($tag:expr, $msg:expr) => {

                    sender.send_message(&OwnedMessage::Text(format!("[{}] {}",$tag ,$msg))).unwrap();

                }
            }

            log!( "conn" );


            for message in reciever.incoming_messages() {
                let message = message.unwrap();

                match  message {
                    OwnedMessage::Close(_) => {

                        send!(OwnedMessage::Close(None));
                        log!("disc");
                        return;

                    }

                    OwnedMessage::Ping(ping) => {

                        log!("ping");
                        send!(OwnedMessage::Pong(ping));

                    }

                    OwnedMessage::Text(text) => {

                        log!("mesg", text);

                        let split = text.split(" ")  ;

                        let mut args: Vec<&str> = split.collect();

                        let mut command = process::Command::new(args.remove(0));

                        command.args(args);


                        let output =  command.output();

                        match output {
                            Ok(x) => {

                                let sout = String::from_utf8(x.stdout).unwrap();
                                let serr = String::from_utf8(x.stderr).unwrap(); 
                                
                                if !sout.is_empty() {
                                    log!("sout", sout);
                                    sendText!("sout".to_string(),sout);
                                }
                                if !serr.is_empty() {
                                    log!("serr", serr);
                                    sendText!("serr".to_lowercase(),serr);
                                }


                            } 

                            Err(e) => {

                                log!("erro",e.to_string());
                                sendText!(format!("failed to run the command:\n\t{}",e.to_string()));

                            }
                            
                        }
                        


                    }


                    _ => {}
                    
                }

            }

        });



    }
}
