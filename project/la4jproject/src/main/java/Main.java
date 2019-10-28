import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import org.la4j.Matrix;
import org.la4j.matrix.dense.Basic2DMatrix;

public class Main {
  public static void main(String[] args) {

    String file = "/Users/sinsakuokazaki/master-thesis/project/data/";

    //
    long loadStart = System.currentTimeMillis();

    double[][] data = CSVload(file + args[0]);
    long loadEnd = System.currentTimeMillis();
    long loadDuration = loadEnd - loadStart;
    System.out.println("Load csv file: " + loadDuration + " milliseconds");
   // System.out.println("m: "+ data.length + " n: " + data[0].length);

    Matrix matrix = new Basic2DMatrix().from2DArray(data);

    long productStart = System.currentTimeMillis();
    Matrix dotProdcut = matrix.multiplyByItsTranspose();
    long productEnd = System.currentTimeMillis();
    long productDuration = productEnd - productStart;
    System.out.println("Dot Product: " + productDuration + " milliseconds");

    //Matrix X = matrix.slice(0, 0, data[0].length - 1, data.length - 1);
    //Matrix Y = matrix.slice(data[0].length - 1, data.length - 1, data[0].length, data.length);


  }

  public static double[][] CSVload(String csvFile) {
    BufferedReader csvReader = null;
    List<double[]> dataDouble = new ArrayList<double[]>();
    try
    {
      String row;
      csvReader = new BufferedReader(new FileReader(csvFile));
      while ((row = csvReader.readLine()) != null) {
        String[] elmStr = row.split(",");
        double[] elmDouble = new double[elmStr.length];
        for (int i = 0; i < elmStr.length; i++) {
          elmDouble[i] = Float.parseFloat(elmStr[i]);
        }
        dataDouble.add(elmDouble);
      }
    }
    catch (Exception e) {
      e.printStackTrace();
    }
    finally {
      try {
        csvReader.close();
      } catch (IOException e) {
        e.printStackTrace();
      }
    }
    double[][] data = new double[dataDouble.size()][];
    for (int i = 0; i < dataDouble.size(); i++) {
      data[i] = dataDouble.get(i);
    }
    return data;
  }
}
