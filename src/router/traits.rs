pub trait PostOfficeTrait<T> {
    fn send_message(message: Box<T>);
}
pub trait SenderReciverTrait<S, R> {
    fn get_sender_tx() -> S;
    fn get_reciver_rx() -> R;
}
