public class FilterVector {
    int size;
    boolean[] data;
    public FilterVector(int size) {
        this.size = size;
        this.data = new boolean[size];
    }

    public FilterVector(boolean[] data) {
        this.size = data.length;
        this.data = data;
    }

    public int getSize() {
        return size;
    }

    public boolean[] getData() {
        return data;
    }

    public boolean get(int i) {
        return data[i];
    }

    public void set(int i, boolean val) {
        data[i] = val;
    }
}
