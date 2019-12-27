package allocation;

public class Customer {
    int totalOrder;
    double weightOrder;
    String zipCode;

    public Customer(int totalOrder, double weightOrder, String zipCode) {
        this.totalOrder = totalOrder;
        this.weightOrder = weightOrder;
        this.zipCode = zipCode;
    }

    public int getTotalOrder() {
        return totalOrder;
    }

    public double getWeightOrder() {
        return weightOrder;
    }

    public String getZipCode() {
        return zipCode;
    }
}

