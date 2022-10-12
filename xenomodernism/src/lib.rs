use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::{
    HttpClient, HttpClientSender, HttpRequest as HttpClientRequest,
};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(serde::Deserialize)]
// CyberCat
struct CyberCat {
    images: Vec<String>,
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct XmActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for XmActor {
    async fn handle_request(&self, ctx: &Context, _req: &HttpRequest) -> RpcResult<HttpResponse> {
        // The new Kung-Fu Kittie
        let prompt: String = format!("cybernetic+vagina+cat");
        // welcome to web9 from outer space 🛸
        let xeno_url = format!("https://lexica.art/api/v1/search?q={}", prompt);

        let response = HttpClientSender::new()
            .request(ctx, &HttpClientRequest::get(&xeno_url))
            .await?;

        // Deserialize JSON to retrieve comic title and img URL
        let cat: CyberCat = serde_json::from_slice(&response.body).map_err(|e| {
            RpcError::ActorHandler(format!("Failed to deserialize CyberCat request: {}", e))
        })?;

        // Format HTTP response body as an HTML string
        let body = format!(
            r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>xm</title>
        </head>
        <body>
            <h1>{}</h1>
            <img src="{}"/>
        </body>
        </html>
            "#,
            "xm", cat.images
        );

        Ok(HttpResponse {
            body: body.as_bytes().to_vec(),
            ..Default::default()
        })
    }
}
