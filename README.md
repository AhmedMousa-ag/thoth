# Thoth

<img src="Assets/thoth.png" alt="Thoth Logo" width="80"/>

Thoth is a Rust-based distributed data computing framework designed for high performance and scalability.
It provides a robust platform for processing large datasets across multiple nodes in a cluster.

## Getting Started

These instructions will help you set up and run Thoth on your local machine or cluster.
Please don't use this in production as it is still in the very early stages of development.

### Prerequisites

- Rust (latest stable version recommended)
- Cargo

### Build

```bash
cargo build --release --target-dir thoth_binary
```

## Usage

To use Thoth, you need to run the binary file generated after building the project. (You can run it on a single machine for testing purposes.)
You can use command `cargo build --release --target-dir thoth_binary` to build the project and then run the binary file located at `thoth_binary/release/thoth`.

Please avoid using `cargo run` for running the project as it is not optimized for performance.


**Example Usage**

Thoth uses gRPC for sending tasks to nodes in the cluster. You can use python for now to communicate until we further expand to other programming languages using the following example.
Note that you need to have the `py_thoth` package installed. You can install it using pip:

```bash
cd py_thoth && pip install -e .
```

Note that you need to have the Thoth binary running on the specified remote addresses before executing the following code. Thoth gRPC uses port `50051` by default.
You can pass multiple remote addresses to distribute the workload across multiple nodes as it will use random load balancing to send tasks to nodes.
It is not important which node you send the task to as all nodes are equal in the cluster except for the node that performs planning for each new task and distributes the workload.

```python
import random
from py_thoth.operations.vector import list_average
from py_thoth.operations.matrices import matrix_multiply
from py_thoth.settings.connections import change_remote_address

# Set remote addresses for Thoth nodes (use localhost for testing)
remote_addresses = ["localhost:50051"]
change_remote_address(remote_addresses)

# Example: Compute average of a large list
list_to_compute = [random.randint(1, 100) for _ in range(1000)]
average_result = list_average(list_to_compute)
print(f"Computed average: {average_result:.2f}")

# Validate result with Python's built-in calculation
expected_average = round(sum(list_to_compute) / len(list_to_compute), 2)
print(f"Expected average: {expected_average:.2f}")
assert round(average_result, 2) == expected_average, "Average result does not match"

# Example: Matrix multiplication
matrix_a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
matrix_b = [[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]
result = matrix_multiply(matrix_a, matrix_b)
print(f"Matrix multiplication result: {result}")

# Validate result with expected output
expected_result = [[58.0, 64.0], [139.0, 154.0]]
print(f"Expected result: {expected_result}")
assert result == expected_result, "Matrix multiplication result does not match"

print("All tests passed successfully!")
```

For API documentation, please refer to the [API Documentation](docs/api.md) document.

## Benchmarks


The following benchmarks were conducted on a machine with the following specifications:
![Benchmark](Assets/benchmark_list_avg.png)


For detailed benchmark results and comparisons, please refer to the [Benchmark Results](docs/benchmarks.md) document.


