public class FilterMatrix {
    int numRows;
    int numColumns;
    boolean[] data;
    public FilterMatrix(int numRows, int numColumns) {
        this.numRows = numRows;
        this.numColumns = numColumns;
        this.data = new boolean[numRows * numColumns];
    }
    public FilterMatrix(boolean[][] arr) {
        this.numRows = arr[0].length;
        this.numColumns = arr.length;
        boolean[] data = new boolean[numRows * numColumns];
        for (int i = 0; i < numColumns; i++) {
            for (int j = 0; j < numRows; j++) {
                data[j + i * numRows] = arr[i][j];
            }
        }
        this.data = data;
    }

    public int getNumRows() {
        return numRows;
    }

    public int getNumColumns() {
        return numColumns;
    }

    public boolean[] getData() {
        return data;
    }

    public void set(int row, int column, boolean val) {
        data[row + column * numRows] = val;
    }
}
