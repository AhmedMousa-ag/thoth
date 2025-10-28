# Thoth vs PySpark Performance Benchmark

## Machine Specifications

The used machines have the following specifications:
- **CPU**: 3 vCPUs
- **RAM**: 3096 MB
- **Disk**: 70 GB

All virtual machines were running Ubuntu 24.04 LTS server edition.
All virtual machines were running on the same host device to ensure consistency in hardware performance.

## Benchmark Overview

The benchmark was conducted on four different dataset sizes:
- **Small**: 50 records
- **Medium**: 5,000 records  
- **Large**: 500,000 records
- **Extra Large**: 5,000,000 records
- **Double Extra Large**: 10,000,000 records

### Cluster Configurations

Two cluster configurations were tested:
1. **Two-Machine Cluster**: 1 master + 1 worker node for spark and 2 distributed nodes for thoth
2. **Three-Machine Cluster**: 1 master + 2 worker nodes for spark and 3 distributed nodes for thoth

## Performance Metrics

Four key operations used for the metrics:
1. **Average**: Calculate the mean for an array
2. **Minimum**: Finding the minimum in an array  
3. **Maximum**: Finding the maximum in an array
4. **Sort Ascending**:Sorting an array in ascending order



## Performance Results

We have performed benchmarks on the two and three machines cluster configuration.

Thoth outperforms PySpark in all tested operations on small to average datasizes and struggles exponentialy with the the increase of the data size.

Clearly Spark sorting algorithm is more optimized for large datasets compared to Thoth.

For the three machines cluster configuration, Thoth shows a potential for better performance with increased nodes, but still needs optimization for increasing data sizes.

### Two-Machine Cluster 

#### Average Calculation

![Average Time Comparison - 2 Machines](./assets/benchmarks/two_machines/avg_time_comparison.png)


##### Minimum Calculation

![Minimum Time Comparison - 2 Machines](./assets/benchmarks/two_machines/min_time_comparison.png)


##### Maximum Calculation

![Maximum Time Comparison - 2 Machines](./assets/benchmarks/two_machines/max_time_comparison.png)


##### Sorting Performance

![Sort Ascending Comparison - 2 Machines](./assets/benchmarks/two_machines/sort_asc_time_comparison.png)



### Three-Machine Cluster 

#### Average Calculation

![Small Dataset Comparison - 3 Machines](./assets/benchmarks/three_machines/avg_time_comparison.png)

#### Minimum Calculation

![Medium Dataset Comparison - 3 Machines](./assets/benchmarks/three_machines/min_time_comparison.png)

#### Maximum Calculation

![Large Dataset Comparison - 3 Machines](./assets/benchmarks/three_machines/max_time_comparison.png)

#### Sorting Calculation

![Sort Ascending Comparison - 3 Machines](./assets/benchmarks/three_machines/sort_asc_time_comparison.png)


### Combined Analysis 

In the combined analysis, the performance of adding extra nodes to Thoth and Spark does increase performance, but Thoth metrices scores aren't stable due to it's thread and concurrency operations.


##### Comparison of small dataset

![Combined Small Dataset](./assets/benchmarks/combined/combined_comparison_small.png)


##### Comparison of medium dataset
![Combined Medium Dataset](./assets/benchmarks/combined/combined_comparison_medium.png)


##### Comparison of large dataset
![Combined Large Dataset](./assets/benchmarks/combined/combined_comparison_large.png)

##### Comparison of extra large dataset
![Combined Extra Large Dataset](./assets/benchmarks/combined/combined_comparison_extra.png)


##### Comparison of double extra large dataset
![Combined Double XL Dataset](./assets/benchmarks/combined/combined_comparison_double.png)

## Future Work

Due to limited time and resources, Thoth I didn't have time to fully optimize Thoth for these benchmarks. Nevertheless, Thoth demonstrates potential in distributed data processing. Further optimization of individual algorithms is necessary to make it competitive with established frameworks like Spark.

Planned future enhancements include support for SQL queries, integration of machine learning libraries, and real-time data processing capabilities. These features aim to improve Thoth's usability and performance across diverse data processing scenarios.
