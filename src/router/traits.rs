pub trait PostOfficeTrait<T> {
    fn send_message(message: Box<T>);
    fn handle_incom_msg(message: Box<T>) -> impl std::future::Future<Output = ()> + Send;
}
pub trait SenderReciverTrait<S, R> {
    fn get_sender_tx() -> S;
    fn get_reciver_rx() -> R;
}
