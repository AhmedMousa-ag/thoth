pub trait PostOfficeTrait<T> {
    fn send_message(message: T);
    fn start_back_office();
}
pub trait SenderReciverTrait<S, R> {
    fn get_sender_tx() -> S;
    fn get_reciver_rx() -> R;
}
