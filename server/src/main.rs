extern crate websocket;
extern crate avr;

use std::thread;

use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;

use avr::emulator;
use avr::assembler;

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();

    for connection in server {
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap(); // Get the request
            let headers = request.headers.clone(); // Keep the headers so we can check them

            request.validate().unwrap(); // Validate the request

            let mut response = request.accept(); // Form a response

            if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
                if protocols.contains(&("rust-websocket".to_string())) {
                    // We have a protocol we want to use
                    response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
                }
            }

            let mut client = response.send().unwrap(); // Send the response

            let ip = client.get_mut_sender()
            .get_mut()
            .peer_addr()
            .unwrap();

            println!("Connection from {}", ip);

            let mut emulator_instance = emulator::Emulator {
                data_memory: emulator::AvrDataMemory {
                    registers: vec![0,2,3,0,0,0,1,5],
                    io: vec![],
                    ram: vec![]
              },
              machine_code: assembler::assemble("")
            };

            let encoded = Message::Text(emulator::serialize(&emulator_instance));
            client.send_message(encoded).unwrap();

            let (mut sender, mut receiver) = client.split();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    Message::Close(_) => {
                        let message = Message::Close(None);
                        sender.send_message(message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    Message::Ping(data) => {
                        let message = Message::Pong(data);
                        sender.send_message(message).unwrap();
                    }
                    Message::Text(data) => {
                        //emulator_instance = emulator::perform_instruction(&emulator_instance, data);
                        //let encoded = Message::Text(emulator::serialize(&emulator_instance));
                        let encoded = Message::Text("hi".to_string());
                        sender.send_message(encoded).unwrap();
                    }
                    _ => {
                        // this is where we are echoing back the message sent to us
                        let message = Message::Text("Should not be here".to_string());
                        sender.send_message(message).unwrap();
                    }
                }
            }
        });
    }
}
