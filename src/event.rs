use crate::model::Device;

#[derive(Debug)]
pub(crate) enum Event {
    DiscoveredDevices(Vec<Device>),
}
