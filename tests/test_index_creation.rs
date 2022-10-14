use elastic4forensics::{IndexBuilder, WithHost};
use elasticsearch::auth::Credentials;
mod common;

#[tokio::test]
async fn test_index_creation() -> Result<(), Box<dyn std::error::Error>> {
    let username = common::credentials::username_from_env();
    let password = common::credentials::password_from_env();
    let credentials = Credentials::Basic(username, password);
    let builder = IndexBuilder::with_name("elastic4forensics_test".to_string())
        .with_host("127.0.0.1")
        .with_port(9200)
        .without_certificate_validation()
        .with_credentials(credentials)
    ;
        
    if let Err(e) = builder.do_build().await { panic!("{e}") }

    assert!(builder.do_index_exists().await?);
    Ok(())
}