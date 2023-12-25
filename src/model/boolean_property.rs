use crate::model::{Common, PropertyType};

#[derive(Debug)]
pub struct BooleanProperty {
    common: Common,
    value: bool,
}

impl BooleanProperty {
    pub fn new(
        name: String,
        readonly: bool,
        property_type: PropertyType,
        external_id: Option<String>,
        value: bool,
    ) -> Self {
        BooleanProperty {
            common: Common::new(name, readonly, property_type, external_id),
            value,
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

    pub fn value(&self) -> bool {
        self.value
    }
}
