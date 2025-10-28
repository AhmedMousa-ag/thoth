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
remote_addresses = ["localhost:50051"] # You can add more addresses for load balancing.
change_remote_address(remote_addresses)
```

This will set the addresses of the Thoth nodes you want to connect to. You can specify multiple addresses for load balancing. If more than one address is provided, Thoth will use random load balancing to distribute the contact point for the cluster on each task.

## Available Operations

The `py_thoth` package currently provides operations for lists and matrices. Below are some of the available operations:

### List Operations (via ThothVector)

Use the `ThothVector` class for vector/list operations.

#### List Average

Calculate the average of a list of numbers.

```python
from py_thoth.operations.vector import ThothVector

lst = [1.0, 2.0, 3.0, 4.0, 5.0]
v = ThothVector(lst)
result = v.list_average()
print(result)  # Output: 3.0
```

#### Sort List

Sort a list in ascending or descending order.

```python
from py_thoth.operations.vector import ThothVector

lst = [5.0, 2.0, 9.0, 1.0, 7.0]
v = ThothVector(lst)
ascending = v.sort_list(ascending=True)
descending = v.sort_list(ascending=False)
print(ascending)   # Output: [1.0, 2.0, 5.0, 7.0, 9.0]
print(descending)  # Output: [9.0, 7.0, 5.0, 2.0, 1.0]
```

#### Maximum and Minimum

Find the maximum or minimum value in a list.

```python
from py_thoth.operations.vector import ThothVector

lst = [5.0, 2.0, 9.0, 1.0, 7.0]
v = ThothVector(lst)
max_value = v.max_list()
min_value = v.min_list()
print(max_value)  # Output: 9.0
print(min_value)  # Output: 1.0
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
from py_thoth.operations.vector import ThothVector
from py_thoth.operations.matrices import matrix_multiply
from py_thoth.settings.connections import change_remote_address

# Configure cluster (single address or a list for load balancing)
change_remote_address(["localhost:50051"]) 

# List operations via ThothVector
data = [5.0, 2.0, 9.0, 1.0, 7.0]
v = ThothVector(data)
print(f"Average: {v.list_average()}")
print(f"Sorted: {v.sort_list(ascending=True)}")
print(f"Max: {v.max_list()}")
print(f"Min: {v.min_list()}")

# Matrix operations
matrix_a = [[1.0, 2.0], [3.0, 4.0]]
matrix_b = [[5.0, 6.0], [7.0, 8.0]]
result = matrix_multiply(matrix_a, matrix_b)
print(f"Matrix product: {result}")
```

## Optional: Performance smoke test

```python
import random
import time
from py_thoth.operations.vector import ThothVector
from py_thoth.settings.connections import change_remote_address

change_remote_address(["localhost:50051"])  # single node for testing

# Large list average
list_to_compute = [float(random.randint(1, 100)) for _ in range(5_000_000)]
vector_obj = ThothVector(list_to_compute)

start_time = time.time()
average_result = vector_obj.list_average()
end_time = time.time()

print(f"Computed average: {average_result:.2f}")
print(f"Time taken: {end_time - start_time:.2f} seconds")

# Validate against Python's built-in calculation
expected_average = round(sum(list_to_compute) / len(list_to_compute), 2)
assert round(average_result, 2) == expected_average
```

