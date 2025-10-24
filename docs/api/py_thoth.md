# PyThoth
PyThoth is a Python interface for interacting with the Thoth distributed computing framework. It provides a set of operations that can be executed on a Thoth cluster.

## Installation
You can install the `py_thoth` package using pip. First, navigate to the `py_thoth` directory in the Thoth repository, then run:

```bash
pip install -e .
```
This will install the package in editable mode, allowing you to make changes to the code if needed.

## Configuration

Before using PyThoth operations, you need to configure the remote addresses of your Thoth cluster:

```python
from py_thoth.settings.connections import change_remote_address

# Configure cluster addresses
remote_addresses = ["localhost:50051", "localhost:50052"]
change_remote_address(remote_addresses)
```

This will set the addresses of the Thoth nodes you want to connect to. You can specify multiple addresses for load balancing. If more than one address is provided, Thoth will use random load balancing to distribute the contact point for the cluster on each task.

## Available Operations

The `py_thoth` package currently provides operations for lists and matrices. Below are some of the available operations:

### List Operations

#### List Average
Calculate the average of a list of numbers.

```python
from py_thoth.operations.vector import list_average

lst = [1, 2, 3, 4, 5]
result = list_average(lst)
print(result)  # Output: 3.0
```

#### Sort List
Sort a list in ascending or descending order.

```python
from py_thoth.operations.vector import sort_list

lst = [5, 2, 9, 1, 7]
ascending = sort_list(lst, ascending=True)
descending = sort_list(lst, ascending=False)
print(ascending)   # Output: [1, 2, 5, 7, 9]
print(descending)  # Output: [9, 7, 5, 2, 1]
```

#### Maximum and Minimum
Find the maximum or minimum value in a list.

```python
from py_thoth.operations.vector import max_list, min_list

lst = [5, 2, 9, 1, 7]
max_value = max_list(lst)
min_value = min_list(lst)
print(max_value)  # Output: 9
print(min_value)  # Output: 1
```

### Matrix Operations

#### Matrix Multiplication
Multiply two matrices.

```python
from py_thoth.operations.matrices import matrix_multiply

matrix_a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
matrix_b = [[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]
result = matrix_multiply(matrix_a, matrix_b)
print(result)  # Output: [[58.0, 64.0], [139.0, 154.0]]
```

## Complete Example

```python
from py_thoth.operations.vector import list_average, sort_list, max_list, min_list
from py_thoth.operations.matrices import matrix_multiply
from py_thoth.settings.connections import change_remote_address

# Configure cluster
remote_addresses = ["localhost:50051"]
change_remote_address(remote_addresses)

# List operations
data = [5, 2, 9, 1, 7]
print(f"Average: {list_average(data)}")
print(f"Sorted: {sort_list(data, ascending=True)}")
print(f"Max: {max_list(data)}")
print(f"Min: {min_list(data)}")

# Matrix operations
matrix_a = [[1.0, 2.0], [3.0, 4.0]]
matrix_b = [[5.0, 6.0], [7.0, 8.0]]
result = matrix_multiply(matrix_a, matrix_b)
print(f"Matrix product: {result}")
```

