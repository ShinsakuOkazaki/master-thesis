package allocation;

import org.apache.log4j.PropertyConfigurator;


public class StringAddition {

    public static void main(String[] args) {

        int size = Integer.parseInt(args[0]);
        boolean initialization = Boolean.parseBoolean(args[1]);

        PropertyConfigurator.configure("log4j.properties");
        System.gc();

        runExperiments(size, initialization);


    }

    public static void runExperiments(int size, boolean initialization) {
        switch (size) {
            case 1:
                AddToArrayList.addition(10, initialization);
                break;

            case 2:
                AddToArrayList.addition(100, initialization);
                break;
            case 3:
                AddToArrayList.addition(1000, initialization);
                break;
            case 4:
                AddToArrayList.addition(10000, initialization);
                break;
        }

    }
}
