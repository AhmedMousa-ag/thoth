from ..configs.env_vars import REMOTE_ADDRESS
import os
import random
from ..configs.config import Config
import uuid
from utils.util import run_client
from py_thoth.proto import mathop_pb2


def change_remote_address(new_address: list[str]):
    """
    Change the remote address for the Thoth service.

    :param new_address: A list of strings representing the new remote addresses including the port.
    """
    separator = ","
    list_as_string = separator.join(new_address)
    os.environ[REMOTE_ADDRESS] = list_as_string
    print(f"Remote address changed to: {new_address}")


class BaseThothObject:
    """
    A class representing a Thoth object with a remote address.

    :param remote_address: A list of strings representing the remote addresses including the port.
    """

    def __init__(
        self,
        data: list[float],
        remote_address: list[str] | None = None,
        fixed_address: bool = True,
        operation_id: str | None = None,
    ):
        self.remote_address = (
            remote_address
            if not fixed_address
            else (
                random.choice(remote_address)
                if remote_address
                else random.choice(Config().get_remote_address())
            )
        )
        self.operation_id = operation_id if operation_id else str(uuid.uuid4())
        self.__insert_data(data)

    def __insert_data(self, data):
        @run_client(self.remote_address)
        def __insert_data_to_thoth(**kwargs):
            stub = kwargs["stub"]

            req = mathop_pb2.AddDataObjectRequest(
                data=data,
                operation_id=self.operation_id,
            )
            res = stub.AddDataObject(req)
            return res

        __insert_data_to_thoth()

        # TODO
        # def __del__(self):
        #     """
        #     Destructor to clean up resources when the object is garbage collected.
        #     """

        #     @run_client(self.remote_address)
        #     def __delete_data_from_thoth(**kwargs):
        #         stub = kwargs["stub"]

        #         req = mathop_pb2.DeleteDataObjectRequest(
        #             operation_id=self.operation_id,
        #         )
        #         stub.DeleteDataObject(req)

        # __delete_data_from_thoth()
