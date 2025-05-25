#[tokio::main]
async fn main() {
    let urls = vec![
        "https://botanicusboutique.com.au/products/birthday-wishes-gift-card",
        "https://www.bottrellaccounting.com.au/roster-template-2/",
        "https://bornprimitive.com.au/products/tidal-bikini-bottom-summer-fields",
        "https://www.bosshardmedical.com.au/shop/bedroom/accessories/manchester/",
        "https://bottega1995.com.au/product/fourmage-gouda/",
        "https://bostonclothing.com.au/products/pink-clover-paisley-print-silk-pocket-square",
        "https://us.boteh.com.au/collections/ss22-la-esencia/products/la-ponche-minimal-short-1",
        "https://botanex.com.au/products/haws-childrens-watering-can",
        "https://borderchronicle.com.au/sport/2022/01/14/report-from-the-dolphins-amateur-swimming-club/",
        "https://borderwatch.com.au/all-digital-editions/tbw-border-watch-friday-11th-april-2025/",
    ];

    let mut set = tokio::task::JoinSet::new();

    for (i, url_str) in urls.iter().enumerate() {
        let url_owned = url_str.to_string();
        set.spawn(scrape::get(i, url_owned));
    }

    while let Some(_) = set.join_next().await {}
}

mod scrape {
    use native_tls::TlsConnector;
    use std::net::ToSocketAddrs;
    use tokio::{io::AsyncWriteExt, net::TcpStream};
    use tokio_native_tls::TlsConnector as TokioTlsConnector;
    use url::Url;

    pub async fn get(_id: usize, url_str: String) {
        let parsed_url = Url::parse(&url_str).unwrap();
        let host = parsed_url.host_str().unwrap();
        let port = parsed_url.port_or_known_default().unwrap();

        let request_target = format!(
            "{}{}",
            parsed_url.path(),
            parsed_url
                .query()
                .map_or(String::new(), |q| format!("?{}", q))
        );

        let host_header_val = host.to_string();
        let addr_str = format!("{}:{}", host, port);
        let addr = addr_str.to_socket_addrs().unwrap().next().unwrap();

        let request_lines = [
            format!("GET {} HTTP/1.1", request_target),
            format!("Host: {}", host_header_val),
            "User-Agent: curl/7.81.0".to_string(),
            "Accept: */*".to_string(),
            "Connection: close".to_string(),
            "".to_string(),
            "".to_string(),
        ];
        let request = request_lines.join("\r\n");

        if parsed_url.scheme() == "https" {
            let native_connector = TlsConnector::builder().build().unwrap();
            let tokio_connector = TokioTlsConnector::from(native_connector);
            let stream = TcpStream::connect(addr).await.unwrap();
            let mut tls_stream = tokio_connector.connect(host, stream).await.unwrap();

            tls_stream.write_all(request.as_bytes()).await.unwrap();
            tls_stream.shutdown().await.unwrap();
        } else if parsed_url.scheme() == "http" {
            let mut stream = TcpStream::connect(addr).await.unwrap();
            stream.write_all(request.as_bytes()).await.unwrap();
            stream.shutdown().await.unwrap();
        }
    }
}
