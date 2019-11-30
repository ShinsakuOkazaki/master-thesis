package allocation;


import org.apache.log4j.Logger;

import java.util.ArrayList;
import java.util.List;

public class AddToArrayList {
    final static Logger logger = Logger.getLogger(AddToArrayList.class);
    public static void addition(int size, boolean initialization) {

        System.out.println("size: " + size + " initialization: " + initialization);
        // Time stamp for initialization part.
        long startInitializeTime = System.nanoTime();

        List<Integer> arrayList;

        // Initialize ArrayList with or without size specification.
        if (initialization) {
            arrayList = new ArrayList<Integer>(size);
        } else {
            arrayList = new ArrayList<Integer>();
        }
        // Elapsed time for initialization.
        long elapsedInitializeTime = System.nanoTime() - startInitializeTime;

        // Time stamp for additional part.
        long startAdditionTime = System.nanoTime();

        // Add elements to ArrayList.
        for (int i = 0; i < size; i ++) {
            arrayList.add(i);
        }

        //Elapsed time for addition part.
        long elapsedAdditionTime = System.nanoTime() - startAdditionTime;

        //Elapsed time for total operation.
        long elapsedTotalTime = System.nanoTime() - startInitializeTime;

        logger.info("[JavaArrayList]#" + initialization + "#" + size + "#" + elapsedInitializeTime + "#" + elapsedAdditionTime + "#" + elapsedTotalTime);
    }
}
