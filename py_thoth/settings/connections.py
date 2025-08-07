from ..configs.env_vars import REMOTE_ADDRESS
import os


def change_remote_address(new_address: list[str]):
    """
    Change the remote address for the Thoth service.

    :param new_address: A list of strings representing the new remote addresses including the port.
    """
    separator = ","
    list_as_string = separator.join(new_address)
    os.environ[REMOTE_ADDRESS] = list_as_string
    print(f"Remote address changed to: {new_address}")
