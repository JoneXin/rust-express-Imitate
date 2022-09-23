use chrono::Utc;

fn main() {
    let now = Utc::now();
    println!("{}", now);
    let header_str = r#"
        X-Powered-By: Express
        Accept-Ranges: bytes
        Cache-Control: public, max-age=0
        Last-Modified: Tue, 08 Mar 2022 13:11:23 GMT
        ETag: W/"44e-17f69a7dce8"
        Content-Type: text/html; charset=UTF-8
        Content-Length: 1102
        Date: Fri, 23 Sep 2022 03:14:57 GMT
        Connection: close"#;
    format!(
        "{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}: {}\r\n",
        "X-Powered-By: Express",
        "Accept-Ranges: bytes",
        "Cache-Control: public, max-age=0",
        "ETag: W / '44e-17f69a7dce8'",
        "Content-Type: application/json; charset=utf-8",
        "Date",
    );
    println!("{}", header_str);
}
