use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv_server::{CommandRequest, CommandResponse};
use kv_server::{Memtable, Service};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建连接地址
    let addr = "127.0.0.1:9527";

    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);

    // 初始化 server 对象
    let server: Service = Service::new(Memtable::new());

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Got a new connection, addr: {}", addr);

        let svc = server.clone();
        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();

            while let Some(Ok(msg)) = stream.next().await {
                info!("Got command msg: {:?}", &msg);
                let res = svc.execute(msg);

                // 首先给客服端返回一个404
                // let mut resp = CommandResponse::default();
                // resp.status = 404;
                // resp.message = "Not Found".into();
                stream.send(res).await.unwrap();
            }
            info!("Connection closed, addr: {}", addr);
        });
    }
}
