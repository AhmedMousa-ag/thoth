pub trait PostOfficeTrait<S, H> {
    fn send_message(message: S);
    fn handle_incom_msg(message: H) -> impl std::future::Future<Output = ()> + Send;
}
pub trait SenderReciverTrait<S, R> {
    fn get_sender_tx() -> S;
    fn get_reciver_rx() -> R;
}
