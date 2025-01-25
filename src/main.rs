use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use env_logger::Env;
use log::info;
mod logger;
mod config_loader;


#[tokio::main]
async fn main() {
    config_loader::init_config("./config.json").unwrap();
    env_logger::init_from_env(Env::default().default_filter_or(config_loader::get_config().unwrap().log_level.clone()));
    let server_address = config_loader::get_config().unwrap().addr.clone();
    // HTTPサーバを起動
    let listener = TcpListener::bind(&server_address).await.unwrap();
    info!("[Start Server] http://{}", server_address);

    // Ctrl+Cシグナルを待ち受ける
    let signal = tokio::signal::ctrl_c();

    // クライアントからの接続を待ち受ける
    tokio::select! {
        _ = async {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                // クライアントとの通信を行う
                tokio::spawn(async move {
                    handle_client(stream).await;
                });
            }
        } => {},
        _ = signal => {
            info!("Server is shutting down");
        },
    }
}

async fn handle_client(mut stream: tokio::net::TcpStream) {
    // クライアントのリクエストを読み込む
    let mut request_buf = [0; 4096];
    let size = stream.read(&mut request_buf).await.unwrap();
    let request = String::from_utf8_lossy(&request_buf[..size]);
    logger::logger(&request);

    // リクエストのパスを解析
    let request_line = request.lines().next().unwrap();
    let path = request_line.split_whitespace().nth(1).unwrap();
    let file_path = if path == "/" {
        "index.html".to_string()
    } else {
        format!("{}.html", &path[1..])
    };

    let response = if tokio::fs::metadata(&file_path).await.is_ok() {
        let contents = tokio::fs::read_to_string(file_path).await.unwrap();
        format!("HTTP/1.1 200 OK\r\n\r\n{}", contents)
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n<h1>404 Not Found</h1>".to_string()
    };

    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}