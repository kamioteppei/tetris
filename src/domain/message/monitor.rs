use crate::domain::draw::container::DrawContainer;

pub enum MonitorMessage {
    Display(usize, DrawContainer),
}
