use arboard::Clipboard;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use local_ip_address::local_ip;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::{fs, task, time};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

async fn routes(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response;
  
    match (req.method(), req.uri().path()) {
      (&Method::GET, "/") => response = Response::new(Body::from("Clipboard Up")),
      (&Method::GET, "/cp") => {
        let mut file = fs::File::open("cp").await.unwrap();
        let mut contents = vec![];
        file.read_to_end(&mut contents).await.unwrap();
        let content = String::from_utf8(contents).unwrap();
        let body: String = std::fs::read_to_string("./src/template").unwrap().parse().unwrap();
        response = Response::builder().status(StatusCode::OK)
                                    .header("Content-Type", "text/html")
                                    .body(Body::from(body.replace("{{content}}", &content)))
                                    .unwrap();
      },
      (_,_) => response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap(),
    }
    
    Ok(response)
  }

#[tokio::main]
async fn main() {    
    task::spawn(async move {
        let mut clipboard = Clipboard::new().unwrap();
        let mut prev_text = clipboard.get_text().unwrap();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("cp")
            .await
            .unwrap();

        loop {
            let text = clipboard.get_text().unwrap();
            if text != prev_text {
                prev_text = text.clone();
                file.write_all(text.as_bytes()).await.unwrap();
            }
            time::sleep(time::Duration::from_millis(400)).await;
        }
    });


    let addr = SocketAddr::from((local_ip().unwrap(), 1312));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(routes)) });
    let server = Server::bind(&addr).serve(make_svc);
    println!("Up on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
