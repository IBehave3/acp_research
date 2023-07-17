use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContainerGetQueryExtractor {
    #[serde(rename(serialize = "containerId", deserialize = "containerId"))]
    pub collection_id: String,
}
