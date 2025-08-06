from ..configs.config import CONFIG


def change_remote_address(new_address: list[str]):
    """
    Change the remote address for the Thoth service.

    :param new_address: A list of strings representing the new remote addresses including the port.
    """
    CONFIG.setup_remote_address(new_address)
    print(f"Remote address changed to: {CONFIG.remote_address}")
