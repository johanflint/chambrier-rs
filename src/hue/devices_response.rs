use chrono::{DateTime, Utc};
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
pub(crate) struct HueError {
    description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub(crate) enum Resource {
    Device(DeviceGet),
    Light(LightGet),
    Button(ButtonGet),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DeviceGet {
    id: String,
    metadata: DeviceMetadata,
    product_data: ProductData,
    services: Vec<ResourceIdentifierGet>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DeviceMetadata {
    archetype: Archetype,
    name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ProductData {
    model_id: String,
    manufacturer_name: String,
    product_name: String,
    product_archetype: Archetype,
    certified: bool,
    software_version: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LightGet {
    id: String,
    owner: ResourceIdentifierGet,
    on: On,
    dimming: Option<Diming>,
    color_temperature: Option<ColorTemperature>,
    color: Option<Color>,
    dynamics: Option<Dynamics>,
    alert: Option<Alert>,
}

impl LightGet {
    pub fn dimming(&self) -> Option<&Diming> {
        self.dimming.as_ref()
    }

    pub fn color_temperature(&self) -> Option<&ColorTemperature> {
        self.color_temperature.as_ref()
    }

    pub fn color(&self) -> Option<&Color> {
        self.color.as_ref()
    }

    pub fn dynamics(&self) -> Option<&Dynamics> {
        self.dynamics.as_ref()
    }

    pub fn alert(&self) -> Option<&Alert> {
        self.alert.as_ref()
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct On {
    on: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Diming {
    brightness: f32,            // >= 0 && <= 100
    min_dim_level: Option<f32>, // >= 0 && <= 100
}

#[derive(Deserialize, Debug)]
pub(crate) struct ColorTemperature {
    mirek: Option<usize>,
    mirek_valid: bool,
    mirek_schema: MirekSchema,
}

impl ColorTemperature {
    pub fn mirek(&self) -> Option<&usize> {
        self.mirek.as_ref()
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct MirekSchema {
    mirek_minimum: usize, // >= 153 && <= 500
    mirek_maximum: usize, // >= 153 && <= 500
}

#[derive(Deserialize, Debug)]
pub(crate) struct Color {
    xy: Xy,
    gamut: Gamut,
    gamut_type: char, // A, B or C
}

#[derive(Deserialize, Debug)]
pub(crate) struct Xy {
    x: f32, // >= 0.0 && <= 1.0
    y: f32, // >= 0.0 && <= 1.0
}

#[derive(Deserialize, Debug)]
pub(crate) struct Gamut {
    red: Xy,
    green: Xy,
    blue: Xy,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Dynamics {
    status: String,             // dynamic_pallette or none
    status_values: Vec<String>, // SupportedDynamicStatus
    speed: f32,                 // >= 0.0 && <= 1.0
    speed_valid: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Alert {
    action_values: Vec<String>, // AlertEffectType
}

#[derive(Deserialize, Debug)]
pub(crate) struct ButtonGet {
    id: String,
    owner: ResourceIdentifierGet,
    metadata: ButtonMetadata,
    button: Button,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ButtonMetadata {
    control_id: u8, // >= 0 && <= 8
}

#[derive(Deserialize, Debug)]
pub(crate) struct Button {
    button_report: Option<ButtonReport>,
    repeat_interval: usize,
    event_values: Vec<ButtonEvent>,
}

impl Button {
    pub fn button_report(&self) -> Option<&ButtonReport> {
        self.button_report.as_ref()
    }

    pub fn event_values(&self) -> Vec<ButtonEvent> {
        self.event_values.clone()
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct ButtonReport {
    updated: DateTime<Utc>,
    event: ButtonEvent,
}

#[derive(Deserialize, Copy, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ButtonEvent {
    InitialPress,
    Repeat,
    ShortRelease,
    LongRelease,
    DoubleShortRelease,
    LongPress,
}

#[derive(Deserialize, PartialEq, Debug)]
pub(crate) struct ResourceIdentifierGet {
    rid: String,
    rtype: String,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Archetype {
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
        let response = fs::read_to_string("tests/resources/devices_response_light.json")?;
        let response = from_str::<DevicesResponse>(&response)?;

        let data = response.data();
        let errors = response.errors();

        assert_eq!(0, errors.len());
        assert_eq!(2, data.len());

        Ok(())
    }

    #[test]
    fn deserializes_a_device() -> Result<(), Box<dyn Error>> {
        let response = fs::read_to_string("tests/resources/devices_response_light.json")?;
        let response = from_str::<DevicesResponse>(&response)?;

        if let Resource::Device(device) = response.data()[0] {
            assert_eq!("90bdce60-3704-470e-be4c-8264f2bc8151", device.id);
            assert_eq!(Archetype::VintageBulb, device.metadata.archetype);
            assert_eq!("Light", device.metadata.name);
            assert_eq!("LWA021", device.product_data.model_id);
            assert_eq!(
                "Signify Netherlands B.V.",
                device.product_data.manufacturer_name
            );
            assert_eq!("Hue filament bulb", device.product_data.product_name);
            assert_eq!(
                Archetype::VintageBulb,
                device.product_data.product_archetype
            );
            assert!(device.product_data.certified);
            assert_eq!("1.104.2", device.product_data.software_version);
            assert_eq!(
                vec![
                    ResourceIdentifierGet {
                        rid: "7a0ece11-0e2d-4bbf-b290-1d575b541533".to_string(),
                        rtype: "zigbee_connectivity".to_string()
                    },
                    ResourceIdentifierGet {
                        rid: "4e5ad66f-633e-4300-84cd-634129fdb451".to_string(),
                        rtype: "light".to_string()
                    },
                    ResourceIdentifierGet {
                        rid: "5d25baca-11e6-4635-91e2-e4db0b538cc9".to_string(),
                        rtype: "taurus_7455".to_string()
                    },
                    ResourceIdentifierGet {
                        rid: "64ac92d6-41b3-4f81-bd6d-315f01dc59c3".to_string(),
                        rtype: "device_software_update".to_string()
                    }
                ],
                device.services
            );
        } else {
            assert!(false, "data[0] is not a Resource::Device");
        }

        Ok(())
    }

    #[test]
    fn deserializes_a_light() -> Result<(), Box<dyn Error>> {
        let response = fs::read_to_string("tests/resources/devices_response_light.json")?;
        let response = from_str::<DevicesResponse>(&response)?;

        if let Resource::Light(light) = response.data()[1] {
            assert_eq!("4e5ad66f-633e-4300-84cd-634129fdb451", light.id);
            assert_eq!(
                ResourceIdentifierGet {
                    rid: "90bdce60-3704-470e-be4c-8264f2bc8151".to_string(),
                    rtype: "device".to_string()
                },
                light.owner
            );
            assert!(light.on.on);
            assert_eq!(24.11, light.dimming().unwrap().brightness);
            assert_eq!(2.0, light.dimming().unwrap().min_dim_level.unwrap());
            assert!(light.color_temperature().unwrap().mirek.is_none());
            assert_eq!(false, light.color_temperature().unwrap().mirek_valid);
            assert_eq!(
                153,
                light
                    .color_temperature()
                    .unwrap()
                    .mirek_schema
                    .mirek_minimum
            );
            assert_eq!(
                500,
                light
                    .color_temperature()
                    .unwrap()
                    .mirek_schema
                    .mirek_maximum
            );
            assert_eq!(0.669, light.color().unwrap().xy.x);
            assert_eq!(0.3251, light.color().unwrap().xy.y);
            assert_eq!(0.675, light.color().unwrap().gamut.red.x);
            assert_eq!(0.322, light.color().unwrap().gamut.red.y);
            assert_eq!(0.409, light.color().unwrap().gamut.green.x);
            assert_eq!(0.518, light.color().unwrap().gamut.green.y);
            assert_eq!(0.167, light.color().unwrap().gamut.blue.x);
            assert_eq!(0.04, light.color().unwrap().gamut.blue.y);
            assert_eq!('B', light.color().unwrap().gamut_type);
            assert_eq!("none", light.dynamics().unwrap().status);
            assert_eq!(vec!["none"], light.dynamics().unwrap().status_values);
            assert_eq!(0.0, light.dynamics().unwrap().speed);
            assert_eq!(false, light.dynamics().unwrap().speed_valid);
        } else {
            assert!(false, "data[1] is not a Resource::Light");
        }

        Ok(())
    }

    #[test]
    fn deserializes_a_button() -> Result<(), Box<dyn Error>> {
        let response = fs::read_to_string("tests/resources/devices_response_button.json")?;
        let response = from_str::<DevicesResponse>(&response)?;

        let data = response.data();
        let errors = response.errors();

        assert_eq!(0, errors.len());
        assert_eq!(5, data.len()); // Device and 4 buttons

        if let Resource::Button(button) = data[1] {
            assert_eq!("9ea998a8-c996-4a8b-a652-cb7baa9d26e5", button.id);
            assert_eq!(
                ResourceIdentifierGet {
                    rid: "e84075f8-023f-43e7-80ea-c0246fdf2835".to_string(),
                    rtype: "device".to_string()
                },
                button.owner
            );
            assert_eq!(1, button.metadata.control_id);
            assert!(button.button.button_report.is_none());
            assert_eq!(800, button.button.repeat_interval);
            assert_eq!(
                vec![
                    ButtonEvent::InitialPress,
                    ButtonEvent::Repeat,
                    ButtonEvent::ShortRelease,
                    ButtonEvent::LongRelease,
                    ButtonEvent::LongPress
                ],
                button.button.event_values()
            );
        } else {
            assert!(false, "data[1] is not a Resource::Button");
        }

        if let Resource::Button(button) = data[2] {
            assert_eq!("cee245f5-db5a-4876-980c-f32d958e2392", button.id);
            assert_eq!(
                DateTime::UNIX_EPOCH,
                button.button.button_report().unwrap().updated
            );
            assert_eq!(
                ButtonEvent::ShortRelease,
                button.button.button_report().unwrap().event
            );
        } else {
            assert!(false, "data[2] is not a Resource::Button");
        }

        Ok(())
    }
}
