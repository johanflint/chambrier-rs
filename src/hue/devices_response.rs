use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct DevicesResponse {
    errors: Vec<HueError>,
    data: Vec<Resource>,
}

impl DevicesResponse {
    pub fn errors(self) -> Vec<HueError> {
        self.errors
    }

    pub fn data(self) -> Vec<Resource> {
        self.data
            .into_iter()
            .filter(|r| !matches!(r, Resource::Unknown))
            .collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct HueError {
    description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Resource {
    Device(Device),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    id: String,
    metadata: DeviceMetadata,
    product_data: ProductData,
    services: Vec<ResourceIdentifierGet>,
}

#[derive(Deserialize, Debug)]
pub struct DeviceMetadata {
    archetype: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductData {
    model_id: String,
    manufacturer_name: String,
    product_name: String,
    product_archetype: String,
    certified: bool,
    software_version: String,
}

#[derive(Deserialize, Debug)]
pub struct ResourceIdentifierGet {
    rid: String,
    rtype: String,
}
