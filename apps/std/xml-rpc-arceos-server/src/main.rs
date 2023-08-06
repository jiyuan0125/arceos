use anyhow::Result;

use xml_rpc_arceos::header;
use xml_rpc_arceos::server::Server;

fn hello_world(_request: &str) -> String {
    let body = r#"<html>
<head>
  <title>Hello, ArceOS</title>
</head>
<body>
  <center>
    <h1>Hello, <a href="https://github.com/rcore-os/arceos">ArceOS</a></h1>
  </center>
  <hr>
  <center>
    <i>Powered by <a href="https://github.com/rcore-os/arceos/tree/main/apps/net/httpserver">ArceOS example HTTP server</a> v0.1.0</i>
  </center>
</body>
</html>
"#;
    format!(header!(), body.len(), body)
}

fn main() -> Result<()> {
    let mut server = Server::bind("0.0.0.0:5555".parse()?)?;
    server.register_handler("/", Box::new(hello_world));
    server.run();
    Ok(())
}
