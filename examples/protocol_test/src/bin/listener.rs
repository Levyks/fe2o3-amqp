use std::time::Duration;

use fe2o3_amqp::acceptor::{
    link::{LinkAcceptor, LinkEndpoint},
    session::{ListenerSessionHandle, SessionAcceptor},
    ConnectionAcceptor, ListenerConnectionHandle,
};
use tokio::net::TcpListener;
use tracing::{instrument, Level};
use tracing_subscriber::FmtSubscriber;

const BASE_ADDR: &str = "localhost:5672";

#[instrument(skip_all)]
async fn session_main(mut session: ListenerSessionHandle) {
    let link_acceptor = LinkAcceptor::new();

    while let Ok(link) = link_acceptor.accept(&mut session).await {
        match link {
            LinkEndpoint::Sender(sender) => {
                let handle = tokio::spawn(async {
                    tracing::info!("Incoming link is connected (remote: receiver, local: sender)");
                    // tokio::time::sleep(Duration::from_millis(1000)).await;
                    sender.close().await.unwrap();
                });
                handle.await.unwrap();
            }
            LinkEndpoint::Receiver(recver) => {
                let handle = tokio::spawn(async {
                    tracing::info!("Incoming link is connected (remote: sender, local: receiver");
                    // tokio::time::sleep(Duration::from_millis(1000)).await;
                    recver.close().await.unwrap();
                });
                handle.await.unwrap();
            }
        }
    }
    session.on_end().await.unwrap();
}

#[instrument(skip_all)]
async fn connection_main(mut connection: ListenerConnectionHandle) {
    let session_acceptor = SessionAcceptor::default();

    while let Ok(session) = session_acceptor.accept(&mut connection).await {
        let _handle = tokio::spawn(session_main(session));
    }
    connection.on_close().await.unwrap();
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let tcp_listener = TcpListener::bind(BASE_ADDR).await.unwrap();
    let connection_acceptor = ConnectionAcceptor::new("test_conn_listener");

    while let Ok((stream, addr)) = tcp_listener.accept().await {
        println!("Incoming connection from {:?}", addr);
        let connection = connection_acceptor.accept(stream).await.unwrap();

        let _handle = tokio::spawn(connection_main(connection));
    }
}
