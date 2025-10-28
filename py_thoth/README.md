# Installation

From current directory run the following command in your python virtual environment.

```sh
pip install -e .
```


## Quick start

```python
from py_thoth.operations.vector import ThothVector
from py_thoth.operations.matrices import matrix_multiply
from py_thoth.settings.connections import change_remote_address

# Configure your Thoth node(s)
change_remote_address(["localhost:50051"])  # or multiple addresses for load balancing

# Vector operations with ThothVector
data = [5.0, 2.0, 9.0, 1.0, 7.0]
v = ThothVector(data)
print("Average:", v.list_average())
print("Ascending:", v.sort_list(ascending=True))
print("Descending:", v.sort_list(ascending=False))
print("Max:", v.max_list())
print("Min:", v.min_list())

# Matrix multiplication
A = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
B = [[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]
print("A x B:", matrix_multiply(A, B))
```

### Optional: performance smoke test

```python
import random
import time
from py_thoth.operations.vector import ThothVector
from py_thoth.settings.connections import change_remote_address

change_remote_address(["localhost:50051"])  # single node is fine for testing

# Large list average
lst = [float(random.randint(1, 100)) for _ in range(5_000_000)]
v = ThothVector(lst)
t0 = time.time()
avg = v.list_average()
dt = time.time() - t0
print(f"Computed average: {avg:.2f}")
print(f"Time taken: {dt:.2f}s")

# Verify correctness against Python baseline
expected = round(sum(lst) / len(lst), 2)
assert round(avg, 2) == expected
```