use std::collections::HashMap;

use thiserror::Error;

use crate::event::Event;
use crate::hue::client::{HueClient, HueClientError};
use crate::hue::devices_response::{
    DeviceGet, HueError, LightGet, Resource, ResourceIdentifierGet, ResourceType,
};
use crate::model::{BooleanProperty, Device, DeviceType, Property, PropertyType};

pub struct HueObserver {
    client: HueClient,
}

impl HueObserver {
    pub fn new(client: HueClient) -> HueObserver {
        HueObserver { client }
    }

    pub async fn observe(&self) -> Result<Event, HueObserverError> {
        let response = self.client.fetch_devices().await?;
        if response.errors().len() > 0 {
            return Err(HueObserverError::FetchDevicesResponse(
                response.take_errors(),
            ));
        }

        let resource_map = response.devices_map();
        let devices = response
            .devices()
            .iter()
            .try_fold(vec![], |devices: Vec<Device>, device| {
                fold_device(devices, device, &resource_map)
            })?;

        Ok(Event::DiscoveredDevices(devices))
    }
}

fn fold_device(
    mut devices: Vec<Device>,
    device: &&DeviceGet,
    resource_map: &HashMap<String, &Resource>,
) -> Result<Vec<Device>, HueObserverError> {
    let properties: HashMap<String, Property> = device
        .services()
        .iter()
        .try_fold(HashMap::new(), |properties, service| {
            fold_services(properties, service, resource_map)
        })?;

    if properties.len() > 0 {
        let device = map_device(device, properties, None);
        devices.push(device);
    }

    Ok(devices)
}

fn fold_services(
    mut properties: HashMap<String, Property>,
    service: &&ResourceIdentifierGet,
    resource_map: &HashMap<String, &Resource>,
) -> Result<HashMap<String, Property>, HueObserverError> {
    match service.rtype() {
        ResourceType::Light => {
            if let Resource::Light(light) = resource_map[service.rid()] {
                properties.extend(&mut map_lights(light).drain());
            } else {
                return Err(HueObserverError::InvalidData);
            }
        }
        _ => {}
    }

    Ok(properties)
}

fn map_device(
    device: &DeviceGet,
    properties: HashMap<String, Property>,
    external_id: Option<String>,
) -> Device {
    Device::new(
        device.id().to_string(),
        DeviceType::Light,
        device.product_data().manufacturer_name().to_string(),
        device.product_data().model_id().to_string(),
        device.product_data().product_name().to_string(),
        device.metadata().name().to_string(),
        properties,
        external_id,
    )
}

fn map_lights(light: &LightGet) -> HashMap<String, Property> {
    let mut properties: HashMap<String, Property> = HashMap::new();
    let on_property = Property::Boolean(BooleanProperty::new(
        "on".to_string(),
        false,
        PropertyType::On,
        None,
        light.on(),
    ));
    properties.insert("on".to_string(), on_property);
    properties
}

#[derive(Error, Debug)]
pub enum HueObserverError {
    #[error(transparent)]
    ClientError(#[from] HueClientError),
    #[error("fetch devices response contains errors")]
    FetchDevicesResponse(Vec<HueError>),
    #[error("invalid data received from the bridge, a service is not pointing to a valid device")]
    InvalidData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs;

    use serde_json::from_str;

    use crate::hue::devices_response::{DevicesResponse, Resource};
    use crate::hue::observer::fold_device;
    use crate::model::{Property, PropertyType};

    #[test]
    fn folds_a_device_with_one_service() -> Result<(), Box<dyn Error>> {
        let response = fs::read_to_string("tests/resources/devices_response_light.json")?;
        let response = from_str::<DevicesResponse>(&response)?;

        if let Resource::Device(device) = response.data()[0] {
            let devices = fold_device(vec![], &device, &response.devices_map())?;
            assert_eq!(1, devices.len());
            assert_eq!(1, devices[0].properties().len());

            if let Property::Boolean(on_property) = &devices[0].properties()["on"] {
                assert_eq!("on", on_property.name());
                assert_eq!(false, on_property.readonly());
                assert_eq!(PropertyType::On, *on_property.property_type());
                assert_eq!(None, on_property.external_id());
                assert_eq!(true, on_property.value());
            } else {
                assert!(false, r#"property["on"] is not a Resource::Device"#);
            }
            let property = &devices[0].properties()["on"];
            assert!(matches!(property, Property::Boolean(_)))
        } else {
            assert!(false, "data[0] is not a Resource::Device");
        }

        Ok(())
    }

    #[test]
    fn folds_a_device_with_no_used_services() -> Result<(), Box<dyn Error>> {
        let response = fs::read_to_string("tests/resources/devices_with_no_services.json")?;
        let response = from_str::<DevicesResponse>(&response)?;

        if let Resource::Device(device) = response.data()[0] {
            let devices = fold_device(vec![], &device, &response.devices_map())?;
            assert_eq!(0, devices.len());
        } else {
            assert!(false, "data[0] is not a Resource::Device");
        }

        Ok(())
    }
}
