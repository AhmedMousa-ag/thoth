pub struct MessageRouter {}
pub trait RoutMessage {
    fn send_message();
}
impl RoutMessage for MessageRouter {
    fn send_message() {}
}
