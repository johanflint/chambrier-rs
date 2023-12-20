use crate::model::{BooleanProperty, NumberProperty};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Device {
    id: String,
    device_type: DeviceType,
    manufacturer: String,
    model_id: String,
    product_name: String,
    name: String,
    properties: HashMap<String, Property>,
    external_id: Option<String>,
}

impl Device {
    pub fn new(
        id: String,
        device_type: DeviceType,
        manufacturer: String,
        model_id: String,
        product_name: String,
        name: String,
        properties: HashMap<String, Property>,
        external_id: Option<String>,
    ) -> Device {
        Device {
            id,
            device_type,
            manufacturer,
            model_id,
            product_name,
            name,
            properties,
            external_id,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn device_type(&self) -> &DeviceType {
        &self.device_type
    }

    pub fn manufacturer(&self) -> &String {
        &self.manufacturer
    }

    pub fn model_id(&self) -> &String {
        &self.model_id
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn properties(&self) -> &HashMap<String, Property> {
        &self.properties
    }

    pub fn external_id(&self) -> Option<&String> {
        self.external_id.as_ref()
    }
}

#[derive(Debug)]
pub enum DeviceType {
    Light,
}

#[derive(Debug)]
pub enum Property {
    Boolean(BooleanProperty),
    Number(NumberProperty),
}

#[derive(Debug)]
pub(in crate::model) struct Common {
    name: String,
    readonly: bool,
    property_type: PropertyType,
    external_id: Option<String>,
}

impl Common {
    pub fn new(
        name: String,
        readonly: bool,
        property_type: PropertyType,
        external_id: Option<String>,
    ) -> Self {
        Common {
            name,
            readonly,
            property_type,
            external_id,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn readonly(&self) -> bool {
        self.readonly
    }

    pub fn property_type(&self) -> &PropertyType {
        &self.property_type
    }

    pub fn external_id(&self) -> Option<&String> {
        self.external_id.as_ref()
    }
}

#[derive(Debug)]
pub enum PropertyType {
    BatteryLevel,
    Brightness,
    Button,
    Color,
    ColorTemperature,
    LightLevel,
    Motion,
    On,
    Temperature,
}
