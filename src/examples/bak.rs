use std::convert::Infallible;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// use wasmedge_wasi_socket::TcpStream;
use tokio::net::TcpStream;

use hyper::service::{make_service_fn, service_fn};
// use hyper::Client;
use hyper::{Body, Request, Response, Server};
// use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() {
  let mut args = env::args();
  let port = args
    .nth(1)
    .unwrap_or("8080".to_owned())
    .parse::<u16>()
    .unwrap();
  // let target = args.next().unwrap_or("127.0.0.1:3000".to_owned());
  println!("listen 0.0.0.0:{port}");

  // let client = Client::new();
  let service = make_service_fn(|socket: &TcpStream| {
    let remote_addr = socket.peer_addr().unwrap();
    async move {
      Ok::<_, Infallible>(service_fn(move |_: Request<Body>| async move {
        Ok::<_, Infallible>(Response::new(Body::from(format!(
          "Hello, {}!",
          remote_addr
        ))))
      }))
    }
  });
  let server = Server::bind(&SocketAddr::new(
    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
    port,
  ))
  .serve(service);
  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }

  /*
  // 创建一个 Tokio 运行时
  let mut rt = Runtime::new().unwrap();

  // 使用 HTTPS 连接器创建一个 Hyper 客户端
  let client = Client::new(); // Client::builder().build(HttpsConnector::new());

  // 创建一个服务函数，用于处理代理请求
  let service = make_service_fn(move |_| {
    let client = client.clone();
    async move {
      Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
        let uri_str = req.uri().to_string();
        let mut parts = uri_str.split("/");
        parts.next();
        let host = parts.next().unwrap_or("");

        // 将请求转发到目标服务器
        let target_uri =
          format!("https://{}{}", host, req.uri().path_and_query().unwrap());
        let target_req = Request::builder()
          .uri(target_uri)
          .method(req.method())
          .headers(req.headers().clone())
          .body(req.into_body())
          .unwrap();

        client.request(target_req).await
      }))
    }
  });

  // 创建一个 HTTP 服务器
  let server = Server::bind(&([0, 0, 0, 0], port).into()).serve(service);

  // 启动服务器
  rt.block_on(server).unwrap();
  */
}
