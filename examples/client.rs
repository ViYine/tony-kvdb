use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv_server::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建连接地址
    let addr = "127.0.0.1:9527";
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    // 使用 AsyncProstStream 创建一个客户端
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();

    // 生成一个 hset 命令
    let cmd = CommandRequest::new_hset("t1", "hello", "world".into());

    // 发送命令
    client.send(cmd).await?;
    if let Some(Ok(resp)) = client.next().await {
        info!("Got resp: {:?}", resp);
    }

    Ok(())
}
