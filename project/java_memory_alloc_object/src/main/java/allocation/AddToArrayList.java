package allocation;



import org.apache.log4j.Logger;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;
import java.nio.charset.Charset;

public class AddToArrayList {
    final static Logger logger = Logger.getLogger(AddToArrayList.class);
    public static void addition(int size, boolean initialization) {

        System.out.println("size: " + size + " initialization: " + initialization);
        // Time stamp for initialization part.
        long startInitializeTime = System.nanoTime();

        List<Customer> arrayList;

        // Initialize ArrayList with or without size specification.
        if (initialization) {
            arrayList = new ArrayList<Customer>(size);
        } else {
            arrayList = new ArrayList<Customer>();
        }
        // Elapsed time for initialization.
        long elapsedInitializeTime = System.nanoTime() - startInitializeTime;



        // Time stamp for additional part.
        long startAdditionTime = System.nanoTime();

        // Create string array
        Customer[] customer_arr = new Customer[size];
        for (int i = 0; i < size; i++) {
            customer_arr[i] = generateRandomCustomer();
        }

        // Add elements to ArrayList.
        for (int i = 0; i < size; i ++) {
            arrayList.add(customer_arr[i]);
        }

        //Elapsed time for addition part.
        long elapsedAdditionTime = System.nanoTime() - startAdditionTime;

        //Elapsed time for total operation.
        long elapsedTotalTime = System.nanoTime() - startInitializeTime;

        logger.info("[JavaArrayList]#" + initialization + "#" + size + "#" + elapsedInitializeTime + "#" + elapsedAdditionTime + "#" + elapsedTotalTime);
    }

    public static Customer generateRandomCustomer() {
        int totalOrder = getRandomIntegerInRange(0, 100);
        double weightOrder = getRandomDoubleRange(0, 100);
        String zipCode = generateRandomString();
        Customer customer = new Customer(totalOrder, weightOrder, zipCode);
        return customer;
    }

    public static int getRandomIntegerInRange(int min, int max) {

        if (min >= max) {
            throw new IllegalArgumentException("max must be greater than min");
        }

        Random r = new Random();
        return r.nextInt((max - min) + 1) + min;
    }

    public static double getRandomDoubleRange(int min, int max) {

        if (min >= max) {
            throw new IllegalArgumentException("max must be greater than min");
        }

        Random r = new Random();
        return (double) min + (max - min) * r.nextDouble();
    }

    public static String generateRandomString() {
        byte[] array = new byte[10]; // length is bounded by 10
        new Random().nextBytes(array);
        String generatedString = new String(array, Charset.forName("UTF-8"));
        return generatedString;
    }
}
