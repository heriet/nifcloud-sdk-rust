use storage::{Client, Config, Error};
use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
use hyper::header::{HeaderMap, HeaderName, HeaderValue};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let access_key_id = std::env::var("NIFCLOUD_ACCESS_KEY_ID").expect("NIFCLOUD_ACCESS_KEY_ID is not defined");
    let secret_access_key = std::env::var("NIFCLOUD_SECRET_ACCESS_KEY").expect("NIFCLOUD_SECRET_ACCESS_KEY is not defined");

    let region = "jp-east-1";
    let host = format!("{}.storage.api.nifcloud.com", region);
    let url = format!("https://{}/?x-id=GetService", host); // GetService

    let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .enable_http1()
        .build();
    let hyper_client = HyperClientBuilder::new().build(https_connector);

    let config = Config::builder()
        .endpoint_url(url.as_str())
        .http_client(hyper_client)
        .build();
    let client = Client::from_conf(config);

    let response = client
        .get_service()
        .customize()
        .mutate_request(move |req| {
            let headers = req.headers_mut();

            let datetime = chrono::Utc::now();
            let x_amz_date = datetime.format("%Y%m%dT%H%M%SZ").to_string();
            headers.insert(
                HeaderName::from_static("x-amz-date"),
                HeaderValue::from_str(x_amz_date.as_str()).unwrap(),
            );

            let body = "";
            let x_amz_content_sha256 = sha256::digest(body);
            headers.insert(
                HeaderName::from_static("x-amz-content-sha256"),
                HeaderValue::from_str(&x_amz_content_sha256).unwrap(),
            );

            let header_map = HeaderMap::from_iter([
                (
                    HeaderName::from_static("x-amz-date"),
                    HeaderValue::from_str(x_amz_date.as_str()).unwrap(),
                ),
                (
                    HeaderName::from_static("x-amz-content-sha256"),
                    HeaderValue::from_str(&x_amz_content_sha256).unwrap(),
                ),
            ]);

            let aws_sign = aws_sign_v4::AwsSign::new(
                "GET",
                url.as_str(),
                &datetime,
                &header_map,
                region,
                &access_key_id,
                &secret_access_key,
                "s3",
                body,
            );
            let signature = aws_sign.sign();

            headers.insert(
                hyper::header::AUTHORIZATION,
                HeaderValue::from_str(&signature).unwrap(),
            );
        })
        .send()
        .await
        .expect("operation failed");

    println!("{:#?}", response);

    Ok(())
}
