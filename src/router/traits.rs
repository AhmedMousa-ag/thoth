pub trait PostOfficeTrait<T> {
    fn send_message(message: T);
    fn start_back_office();
}
