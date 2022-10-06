use anyhow::Result;
use elasticsearch::{
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    Elasticsearch, cat::CatIndicesParts, cert::CertificateValidation, auth::Credentials, indices::IndicesCreateParts,
};
use serde_json::Value;

pub struct IndexBuilder {
    host: Option<String>,
    port: Option<u16>,
    index_name: String,
    do_certificate_validation: bool,
    credentials: Option<Credentials>
}

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: u16 = 9200;

pub trait WithHost<T> {
    fn with_host(self, host: T) -> Self;
}

impl IndexBuilder {
    pub fn with_name(index_name: String) -> Self {
        Self {
            host: None,
            port: None,
            index_name,
            do_certificate_validation: true,
            credentials: None,
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn without_certificate_validation(mut self) -> Self {
        self.do_certificate_validation = false;
        self
    }

    pub fn with_credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    pub fn host(&self) -> &str {
        match self.host.as_ref() {
            Some(h) => h,
            None => DEFAULT_HOST,
        }
    }

    pub fn port(&self) -> u16 {
        match self.port.as_ref() {
            Some(p) => *p,
            None => DEFAULT_PORT,
        }
    }

    pub fn index_name(&self) -> &str {
        &self.index_name
    }

    pub async fn index_exists(&self) -> Result<bool> {
        let client = self.create_client()?;
        self.client_has_index(&client).await
    }

    pub async fn create_if_not_exists(&self) -> Result<()> {
        let client = self.create_client()?;

        if ! self.client_has_index(&client).await? {
            let parts = IndicesCreateParts::Index(&self.index_name);
            let response = client
                .indices()
                .create(parts)
                .send()
                .await?;
            response.error_for_status_code_ref()?;
        }
        Ok(())
    }

    fn create_client(&self) -> Result<Elasticsearch> {
        let url = Url::parse(&format!("https://{}:{}", self.host(), self.port()))?;
        let conn_pool = SingleNodeConnectionPool::new(url);
        let mut transport_builder = TransportBuilder::new(conn_pool)
            .cert_validation(
                if self.do_certificate_validation { CertificateValidation::Default }
                else { CertificateValidation::None }
            )
            .disable_proxy();
        
        if let Some(credentials) = &self.credentials {
            transport_builder = transport_builder.auth(credentials.clone());
        }
        let transport = transport_builder.build()?;
        Ok(Elasticsearch::new(transport))
    }

    async fn client_has_index(&self, client: &Elasticsearch) -> Result<bool> {
        let response = client
            .cat()
            .indices(CatIndicesParts::Index(&["*"]))
            .format("json")
            .send()
            .await?;
        response.error_for_status_code_ref()?;
        
        if response.content_length().unwrap_or(0) == 0 {
            Ok(false)
        } else {
            let response_body = response.json::<Value>().await?;

            match response_body.as_array() {
                None => Ok(false),
                Some(body) => 
                    Ok(body.iter().any(|r| *r["index"].as_str().unwrap() == self.index_name))
            }
        }
    }
}

impl WithHost<String> for IndexBuilder {
    fn with_host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
}


impl WithHost<&str> for IndexBuilder {
    fn with_host(mut self, host: &str) -> Self {
        self.host = Some(host.to_owned());
        self
    }
}
