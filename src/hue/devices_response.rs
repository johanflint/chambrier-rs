use serde::Deserialize;

// Names come from the [Hue API v2](https://developers.meethue.com/develop/hue-api-v2/api-reference/#resource).

#[derive(Deserialize, Debug)]
pub(crate) struct DevicesResponse {
    errors: Vec<HueError>,
    data: Vec<Resource>,
}

impl DevicesResponse {
    pub fn errors(&self) -> Vec<&HueError> {
        self.errors.iter().collect()
    }

    pub fn data(&self) -> Vec<&Resource> {
        self.data
            .iter()
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
    Device(DeviceGet),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub struct DeviceGet {
    id: String,
    metadata: DeviceMetadata,
    product_data: ProductData,
    services: Vec<ResourceIdentifierGet>,
}

#[derive(Deserialize, Debug)]
pub struct DeviceMetadata {
    archetype: Archetype,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductData {
    model_id: String,
    manufacturer_name: String,
    product_name: String,
    product_archetype: Archetype,
    certified: bool,
    software_version: String,
}

#[derive(Deserialize, Debug)]
pub struct ResourceIdentifierGet {
    rid: String,
    rtype: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Archetype {
    Bollard,
    BridgeV2,
    CandleBulb,
    CeilingRound,
    CeilingSquare,
    ChristmasTree,
    ClassicBulb,
    DoubleSpot,
    FlexiableLamp,
    FloodBulb,
    FloorLantern,
    FloorShade,
    GroundSpot,
    HueBloom,
    HueCentris,
    HueGo,
    HueIris,
    HueLightstrip,
    HueLightstripTv,
    HuePlay,
    HueSigne,
    HueTube,
    LusterBulb,
    PendantLong,
    PendantRound,
    Plug,
    RecessedCeiling,
    RecessedFloor,
    SingleSpot,
    SpotBulb,
    SultanBulb,
    TableShade,
    TableWash,
    UnknownArchetype,
    VintageBulb,
    WallLantern,
    WallShade,
    WallSpot,
    WallWasher,
    #[serde(other)]
    UnrecognizedArchetype,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;
    use std::error::Error;
    use std::fs;

    #[test]
    fn deserializes_a_response_with_data() -> Result<(), Box<dyn Error>> {
        let response = fs::read_to_string("tests/resources/devices_response.json")?;
        let response = from_str::<DevicesResponse>(&response)?;

        let data = response.data();
        let errors = response.errors();

        assert_eq!(0, errors.len());
        assert_eq!(1, data.len());

        Ok(())
    }
}
