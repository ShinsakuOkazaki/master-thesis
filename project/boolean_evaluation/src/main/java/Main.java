import java.util.HashMap;

public class Main {
    public static void main(String[] args) {

    }

    public static int countEval(String s, boolean result, HashMap<String, Integer> map) {
        if (s.length() == 0) return 0;
        if (s.length() == 1) return stringToBoolean(s) == result ? 1 : 0;
        if (map.containsKey(result + s)) return map.get(result + s);
        int ways = 0
        for (int i = 1; i < s.length(); i+=2) {
            char operator = s.charAt(i);
            String left = s.substring(0, i);
            String right = s.substring(i+1, s.length());

            int leftTrue = countEval(left, true, map);
            int leftFalse = countEval(left, false, map);
            int rightTrue = countEval(right, true, map);
            int rightFalse = countEval(left, false, map);
            int total = (leftTrue + leftFalse) * (rightTrue * rightFalse);
            int totalTrue = 0;
            if (operator == '|') {
                totalTrue = leftTrue * rightFalse +
                                leftFalse * rightTrue +
                                leftTrue * leftTrue;
            } else if (operator == '&') {
                totalTrue = leftTrue * rightTrue;
            } else if (operator == '^') {
                totalTrue = leftTrue * rightFalse +
                                leftFalse * rightTrue;
            }

            int subways = result ? totalTrue : total - totalTrue;
            ways += subways;
        }
        map.put(result + s, ways);
        return ways;
    }
    public static boolean stringToBoolean(String s) {
        if (s.equals("0")) {
            return false;
        } else {
            return true;
        }
    }
}
