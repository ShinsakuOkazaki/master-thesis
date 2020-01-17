import no.uib.cipr.matrix.*;
import no.uib.cipr.matrix.Vector;

import java.util.*;
import java.util.function.Supplier;

public class Kmeans {
    int nCluster;
    int maxIter;
    int randomState;

    public Kmeans(int nCluster, int maxIter, int randomState) {
        this.nCluster = nCluster;
        this.maxIter = maxIter;
        this.randomState = randomState;
    }

    private static int[] getRandomIntegersRange(int min, int max, int size) {

        if (min >= max) {
            throw new IllegalArgumentException("max must be greater than min");
        }

        Random r = new Random();
        Set<Integer> generated = new HashSet<Integer>();
        while (size < generated.size()) {
            int element = r.nextInt((max - min) + 1) + min;
            generated.add(element);
        }
        int[] res = new int[size];
        int i = 0;
        for (int e : generated) {
            res[i] = e;
            i++;
        }
        return res;
    }

    private Matrix initializeCentroid(DenseMatrix X) {
        int[] randomIdx = getRandomIntegersRange(0, X.numColumns(), nCluster);
        Matrix centroids = Matrices.getSubMatrix(X, randomIdx, Matrices.index(0, X.numColumns()));
        return centroids;
    }

    private DenseMatrix computeCentroids(DenseMatrix X, DenseVector labels) {
        int numColumns = X.numColumns();

        double[] contents = new double[nCluster * numColumns]

        for (int k = 0; k < nCluster; k++) {
            FilterVector filter = getFilterByCondition(labels, k);
            DenseMatrix filtered = sliceByFilter(X, filter);
            DenseVector mean = mean(X, 1);
            for (int c = 0; c < numColumns; c++) {
                contents[k + c* nCluster] = mean.getData()[c];
            }
        }

        DenseMatrix res = new DenseMatrix(nCluster, numColumns, contents, true);
        return res;
    }

    public DenseVector sliceRow(DenseMatrix X, int row) {
        double[] content =  X.getData();
        int numRows = X.numRows();
        int numColumns = X.numColumns();
        double[] arr = new double[numColumns];
        for (int c = 0; c < numColumns; c++) {
            arr[c] = content[row + c * numRows];
        }
        DenseVector res = new DenseVector(arr, true);
        return res;
    }

    public DenseMatrix sliceByFilter(DenseMatrix X, FilterVector filter) {
        int numRows = X.numRows();
        int numColumns = X.numColumns();
        List<Integer> index = new ArrayList<Integer>();
        for (int c = 0; c < numColumns; c++) {
            if (filter.get(c)) {
                index.add(c);
            }
        }
        double[] arr = new double[index.size() * numColumns];
        for (int i : index) {
            for (int j = 0; j < numRows; j++) {
                arr[j + i * numRows] = X.get(i, j);
            }
        }
        DenseMatrix res = new DenseMatrix(index.size(), numColumns, arr, true);
        return res;
    }

    public FilterVector getFilterByCondition(DenseVector label ,int k) {
        FilterVector res = new FilterVector(label.size());
        for (int i = 0; i < label.size(); i++) {
            res.set(i, (label.get(i) == k));
        }
        return res;
    }

    public DenseVector mean(DenseMatrix X, int axis) {
        int numRows = X.numRows();
        int numColumns = X.numColumns();
        double[] arr;
        DenseVector res;
        if (axis == 1) {
            arr= new double[numColumns];
            for (int i = 0; i < numColumns; i++) {
                for (int j = 0; j < numRows; j++) {
                    arr[i] += X.get(j, i) / numRows;
                }
            }
            res = new DenseVector(arr, true);
        } else {
            arr = new double[numRows];
            for (int i = 0; i < numRows; i++) {
                for (int j = 0; j < numColumns; j++) {
                    arr[i] += X.get(i, j) / numColumns;
                }
            }
            res = new DenseVector(arr, true);
        }
        return res;
    }
}
