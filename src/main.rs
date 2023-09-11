use std::time::Duration;

fn main() {
    let (tx_server, mut rx_client) = tokio::sync::mpsc::channel(1);
    let (tx_client, mut rx_server) = tokio::sync::mpsc::channel(1);
    // server
    let server_handle = std::thread::spawn(move || loop {
        tx_server.blocking_send("hi from server").unwrap();
        let v = rx_server.blocking_recv().unwrap();
        println!("Recv from client: {}", v);
        std::thread::sleep(Duration::from_secs(1));
    });
    // clinet
    let client_handle = std::thread::spawn(move || loop {
        let v = rx_client.blocking_recv().unwrap();
        println!("Recv from server: {}", v);
        std::thread::sleep(Duration::from_secs(1));
        tx_client.blocking_send("hi from client").unwrap();
    });

    server_handle.join().unwrap();
    client_handle.join().unwrap();
}
