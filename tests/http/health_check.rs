use reqwest::Method;

use crate::TestServer;

#[tokio::test]
async fn server_responds() {
    let server = TestServer::start().await.unwrap();
    let res = server
        .call(Method::GET, "/api/health_check")
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
}
