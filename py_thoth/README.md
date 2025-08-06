# Installation

From current directory run the following command in your python virtual environment.

```sh
pip install -e .
```

# Example

An example code of how to work with it as follows:

```python
from py_thoth.operations.vector import list_average
from py_thoth.settings.connections import change_remote_address

change_remote_address(["127.0.0.1:50051", "localhost:50051"])
print(list_average([5, 8, 9, 7, 6, 4, 2]))
```