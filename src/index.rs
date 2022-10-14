use anyhow::{Result, bail};
use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use elasticsearch::{
    http::request::{Body, JsonBody, NdBody},
    Bulk, BulkParts, Elasticsearch,
};
use serde_json::Value;

pub struct Index {
    name: String,
    client: Elasticsearch,

    cache_size: usize,
    document_cache: Option<Vec<Value>>,
}

impl Index {
    pub fn new(name: String, client: Elasticsearch) -> Self {
        Self {
            name,
            client,
            cache_size: 1000,
            document_cache: Some(Vec::new()),
        }
    }

    pub fn add_bulk_document(&mut self, document: Value) -> Result<()> {
        if let Some(c) = self.document_cache.as_mut() {
            c.push(document)
        }

        if self.document_cache.as_ref().unwrap().len() >= self.cache_size {
            self.flush()
        } else {
            Ok(())
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        let parts = BulkParts::Index(&self.name);
        let items = self
            .document_cache
            .replace(Vec::new())
            .unwrap()
            .into_iter()
            .map(JsonBody::new)
            .collect();
        let bulk = Bulk::<JsonBody<Value>>::new(self.client.transport(), parts).body(items);


        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        
        let response = rt.block_on(bulk.send())?;

        if ! response.status_code().is_success() {
            bail!("error {} while sending response", response.status_code());
        }
        Ok(())
    }

    pub fn set_cache_size(&mut self, cache_size: usize) {
        if self.cache_size > cache_size {
            self.flush();
        }
        self.cache_size = cache_size;
    }

    pub fn cache_size(&self) -> usize {
        self.cache_size
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
