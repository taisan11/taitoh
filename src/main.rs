use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use env_logger::Env;
mod logger;

const SERVER_ADDRESS: &str = "127.0.0.1:8888";

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // HTTPサーバを起動
    println!("[HTTPサーバを起動] http://{}", SERVER_ADDRESS);
    let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    // Ctrl+Cシグナルを待ち受ける
    let signal = tokio::signal::ctrl_c();

    // クライアントからの接続を待ち受ける
    tokio::select! {
        _ = async {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                // println!("クライアントが接続しました。");
                // クライアントとの通信を行う
                tokio::spawn(async move {
                    handle_client(stream).await;
                });
            }
        } => {},
        _ = signal => {
            println!("Ctrl+Cが押されました。サーバをシャットダウンします。");
        },
    }
}

async fn handle_client(mut stream: tokio::net::TcpStream) {
    // クライアントのリクエストを読み込む
    let mut request_buf = [0; 4096];
    let size = stream.read(&mut request_buf).await.unwrap();
    let request = String::from_utf8_lossy(&request_buf[..size]);
    // logger::logger(&request);

    // リクエストのパスを解析
    let request_line = request.lines().next().unwrap();
    let path = request_line.split_whitespace().nth(1).unwrap();
    let file_path = if path == "/" {
        "index.html".to_string()
    } else {
        format!("{}.html", &path[1..])
    };

    let response = if Path::new(&file_path).exists() {
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        format!("HTTP/1.1 200 OK\r\n\r\n{}", contents)
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n<h1>404 Not Found</h1>".to_string()
    };

    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}