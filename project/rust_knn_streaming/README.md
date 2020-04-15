# Concept
This experiment is to assess importance of careful memory management in Machine Learning Algorithm for Big Data.
We implements K-nearest-neighbors (Knn) algorithm to perform document classification. Wikipedia data set is used in this experiment.

The Knn algorithm is developed in multithreading and batch processing assuming stream proccessing environment. It has 2 main steps: preprocessing, matrix-computation.
Before these steps, we prepare partitions of train and test file processed in each thread specifying size of batch. After the main steps, we combine predictions.

In the preprocessing step, the partition of file is loaded and Term-Trequencys (TFs) are calculated. 
Stop-words are eliminated and the number of feature (word) is parameterized. This proess includes String vector manipulation where generates many intermediate data structures. 
The same String are used in different data structure many times. Therefore, we can deep-copy the String elements or get reference with Arc. 
We evaluate which approach is efficient in preprocessing step. This step ends when we create Tfs matrixs and category-encoded vectors.

In the matrix-comupation step, we calculate cosine similarity between train and test Tfs matrix. Then, selections of nearest neighbors are performed by alogorithm 
with BinaryHeap. Finally, selections of category id are performed. This step is CPU bound and also memory bound if the size of batch and number of feature is large.
We control batch size and number of feature in Tf matrix to assess iterative memory management for contiguous memory in ML algorithm.

# Run Algorightm
```
./run.sh
```

