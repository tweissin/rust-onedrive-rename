use reqwest::{Client, Method};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn get_access_token(tenant: String, client_id: String, client_secret: String) -> Result<String> {
    let client = Client::new();
    let host = "login.microsoftonline.com";
    let scope = "https://graph.microsoft.com/.default";
    let grant_type = "client_credentials";

    let url_string = format!("https://{}/{}/oauth2/v2.0/token",
                             host, tenant);

    let body = format!("client_id={}&client_secret={}&scope={}&grant_type={}",
                       client_id, client_secret, scope, grant_type);

    // build the request
    let req = client.request(Method::POST, &url_string ).body(body);

    // send the request and wait for it to return
    let res = req.send().await?;
    let body = res.bytes().await?;

    let v = body.to_vec();
    let s = String::from_utf8_lossy(&v);
    Ok(s.to_string())
}
