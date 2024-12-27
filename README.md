## Hierarchical Mixture of Experts (HME)
This toolbox provides hierarchical clustering using the Mixture of Experts model.  
## 2. Description

The HME Toolbox implements the Hierarchical Mixture of Experts (HME) algorithm, designed to handle clustering of biological entities 
like DNA sequences. The toolbox is implemented in Rust for performance and modularity. 
It is intended for researchers and developers working with biological data that requires clustering, dimensionality reduction, 
or similar analysis tasks.

## 3. Features

- Implements Hierarchical Mixture of Experts (HME) for clustering
- Optimized for biological data (e.g., DNA sequences)
- Fast and memory-efficient implementation in Rust
- Modular architecture with easy-to-use APIs
- Includes tools for benchmarking and visualizing clusters

## 4. Installation

To get started with HME Toolbox, follow these steps:

### Clone the repository:
```bash
git clone https://github.com/vliviu/hme_toolbox_RUST.git
cd hme_toolbox
```

Build the project:
Make sure you have Rust installed on your system (visit rust-lang.org for instructions).

Then, build and run the project:

cargo build
cargo run

### 5. **Usage**
Provide examples or instructions on how to use your tool, including commands, functions, or other relevant details.

```markdown
## Usage

To use the HME Toolbox, simply call the `cluster_sequences` function to begin clustering DNA sequences.

### Example:
```rust
use hme_toolbox::clustering::cluster_sequences;

fn main() {
    let sequences = vec!["ACTG", "GCTA", "AGCT"];
    let clusters = cluster_sequences(&sequences);
    println!("{:?}", clusters);
}



### 6. **Contributing**

We welcome contributions! If you'd like to contribute to the project, please fork the repository and create a pull request with your changes. 
For major changes, please open an issue to discuss them first.

### 7. **LICENSE**
## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


### Steps to Contribute:
1. Fork the repository
2. Clone your fork to your local machine
3. Create a new branch
4. Make your changes
5. Push your changes to your fork
6. Open a pull request


## Contact

For any questions or suggestions, please reach out to liviu.vladutul@gmail.com].


HME Toolbox Rust Architecture:
├── hme_toolbox
│   ├── Cargo.toml              # Project metadata and dependencies
│   ├── src
│   │   ├── main.rs             # Entry point, handles CLI or basic app structure
│   │   ├── lib.rs              # Core library module, exposing public API
│   │   ├── clustering
│   │   │   ├── mod.rs          # Module for clustering logic
│   │   │   ├── hme.rs          # Hierarchical Mixture of Experts (HME) algorithm
│   │   │   ├── kmeans.rs       # Optional: K-means baseline for comparison
│   │   ├── data
│   │   │   ├── mod.rs          # Data processing module
│   │   │   ├── fasta.rs        # FASTA/sequence parser (implemented)
│   │   │   ├── csv.rs          # CSV parser for additional metadata
│   │   ├── utils
│   │   │   ├── mod.rs          # Utility functions (e.g., distance metrics, helper methods)
│   │   │   ├── math.rs         # Mathematical operations (matrix, vector operations)
│   │   │   ├── parallel.rs     # Parallel processing utilities
│   │   ├── evaluation
│   │   │   ├── mod.rs          # Model evaluation
│   │   │   ├── metrics.rs      # Clustering quality metrics
│   │   │   ├── visualization.rs # Optional: Visualization module
│   │   ├── io
│   │   │   ├── mod.rs          # I/O operations
│   │   │   ├── reader.rs       # File reading (generic)
│   │   │   ├── writer.rs       # Writing output files
│   ├── tests                  # Integration and unit tests
│   │   ├── clustering_tests.rs
│   │   ├── data_tests.rs       # FASTA parser test implemented
│   │   ├── evaluation_tests.rs
│   │
│   ├── examples               # Example usage files and scripts
│   │   ├── simple_cluster.rs
│   │   ├── benchmark.rs
│
└── README.md                  # Documentation and instructions


## Toolbox Architecture

The `hme_toolbox` Rust project is organized into several modules that each serve specific functionalities. Below is an overview of the key components and their roles:

### Modules

1. **Clustering**
   - Contains the logic for the HME (Hierarchical Mixture of Experts) clustering algorithm.
   - Main function: `HMEClustering`, `HierarchicalCluster`
   - Handles the clustering of biological data (e.g., DNA sequences).

2. **Plotting**
   - Provides tools for visualizing clusters.
   - Main function: `plot_clusters`
   - Generates visualizations to represent clustered data.

3. **Parser**
   - Includes utilities for parsing biological sequence files (e.g., FASTA format).
   - Main function: `parse_fasta`
   - Parses sequence data into a format suitable for clustering.

4. **Benchmark**
   - Used for benchmarking the performance of clustering algorithms.
   - Main function: `benchmark_clustering`
   - Measures and reports the execution time and performance of clustering.

### Directory Structure

```plaintext
src/
│
├── clustering/          # Contains HME clustering logic
│   ├── mod.rs
│   ├── hme.rs           # Core clustering algorithm
│   └── hierarchical.rs  # Hierarchical clustering implementation
│
├── plotter/             # Contains visualization tools
│   └── mod.rs
│
├── parser/              # Parsing logic for biological data
│   └── mod.rs
│
├── benchmark/           # Performance benchmarking tools
│   └── mod.rs
│
└── main.rs              # Entry point for the application
