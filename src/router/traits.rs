pub trait PostOfficeTrait<S> {
    fn send_message(message: S);
    fn handle_incom_msg(message: Option<Vec<u8>>);
}
pub trait SenderReciverTrait<S, R> {
    fn get_sender_tx() -> S;
    fn get_reciver_rx() -> R;
}
