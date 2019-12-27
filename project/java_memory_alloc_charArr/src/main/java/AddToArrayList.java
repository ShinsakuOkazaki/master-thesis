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

        List<char[]> arrayList;

        // Initialize ArrayList with or without size specification.
        if (initialization) {
            arrayList = new ArrayList<char[]>(size);
        } else {
            arrayList = new ArrayList<char[]>();
        }
        // Elapsed time for initialization.
        long elapsedInitializeTime = System.nanoTime() - startInitializeTime;



        // Time stamp for additional part.
        long startAdditionTime = System.nanoTime();

        // Create string array
        char[][] char_arr_arr = new char[size][10];
        for (int i = 0; i < size; i++) {
            char_arr_arr[i] = generateRandomChar();
        }

        // Add elements to ArrayList.
        for (int i = 0; i < size; i ++) {
            arrayList.add(char_arr_arr[i]);
        }

        //Elapsed time for addition part.
        long elapsedAdditionTime = System.nanoTime() - startAdditionTime;

        //Elapsed time for total operation.
        long elapsedTotalTime = System.nanoTime() - startInitializeTime;

        logger.info("[JavaArrayList]#" + initialization + "#" + size + "#" + elapsedInitializeTime + "#" + elapsedAdditionTime + "#" + elapsedTotalTime);
    }

    public static char[] generateRandomChar() {
        byte[] array = new byte[10]; // length is bounded by 10
        new Random().nextBytes(array);
        String generatedString = new String(array, Charset.forName("UTF-8"));
        char[] char_arr = generatedString.toCharArray();
        return char_arr;
    }
}
