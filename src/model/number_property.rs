use crate::model::{Common, PropertyType};

#[derive(Debug)]
pub struct NumberProperty {
    common: Common,
    unit: Unit,
    value: Option<usize>,
    minimum: Option<usize>,
    maximum: Option<usize>,
}

impl NumberProperty {
    pub fn new(
        name: String,
        readonly: bool,
        property_type: PropertyType,
        external_id: Option<String>,
        unit: Unit,
        value: Option<usize>,
        minimum: Option<usize>,
        maximum: Option<usize>,
    ) -> Self {
        NumberProperty {
            common: Common::new(name, readonly, property_type, external_id),
            unit,
            value,
            minimum,
            maximum,
        }
    }

    pub fn name(&self) -> &String {
        &self.common.name()
    }

    pub fn readonly(&self) -> bool {
        self.common.readonly()
    }

    pub fn property_type(&self) -> &PropertyType {
        &self.common.property_type()
    }

    pub fn external_id(&self) -> Option<&String> {
        self.common.external_id()
    }

    pub fn unit(&self) -> &Unit {
        &self.unit
    }

    pub fn value(&self) -> Option<usize> {
        self.value
    }

    pub fn minimum(&self) -> Option<usize> {
        self.minimum
    }

    pub fn maximum(&self) -> Option<usize> {
        self.maximum
    }
}

#[derive(Debug)]
pub enum Unit {
    Percentage,
    Lumen,
    Celcius,
    Kelvin,
}
