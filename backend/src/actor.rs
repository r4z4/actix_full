use std::{net::TcpStream, io::Write};
use futures_util::io;
use tokio::sync::{mpsc, oneshot};
use std::io::Read;
struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    connection: TcpStream,
}

enum ActorMessage {
    SendMessage {
        message: String,
        respond_to: oneshot::Sender<u32>,
    },
}

impl MyActor {
    async fn handle_message(&mut self, msg: ActorMessage) -> io::Result<()> {
        match msg {
            ActorMessage::SendMessage { message, respond_to } => {
                dbg!(message.clone());
                self.connection.write_all(message.as_bytes())?;
                //let response = self.connection.read(&mut [0; 128])?;
                let response = 99;
                let _ = respond_to.send(response.try_into().unwrap());
                Ok(())
            }
        }
    }
}

async fn run_my_actor(mut actor: MyActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await.unwrap();
    }
}

#[derive(Clone)]
pub struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl MyActorHandle {
    pub fn new(conn: TcpStream) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = MyActor { receiver: receiver, connection: conn };
        tokio::spawn(run_my_actor(actor));
        println!("actor spawned");

        Self { sender }
    }
}

impl MyActorHandle {
    pub async fn send_message(&self, msg: String) -> u32 {
        let (send, recv) = oneshot::channel();
        println!("sending message => {msg}");
        let msg = ActorMessage::SendMessage {
            respond_to: send,
            message: msg,
        };
        // Ignore the errors. If this send fails, so does recv.await 
        // below. There is no reason to check the failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}