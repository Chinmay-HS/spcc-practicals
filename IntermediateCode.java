import java.util.*;

public class IntermediateCode {
    static int tempCount = 1;
    static Stack<String> vals = new Stack<>();
    static Stack<Character> ops = new Stack<>();
    static List<String[]> code = new ArrayList<>(); // Stores {op, arg1, arg2, result}

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        System.out.print("Enter expression: ");
        String expr = sc.nextLine().replaceAll("\\s+", ""); 
        sc.close();

        try {
            for (String tk : expr.split("(?<=[-+*/()])|(?=[-+*/()])")) {
                if (tk.isEmpty()) continue;
                char c = tk.charAt(0);

                if (Character.isLetterOrDigit(c)) vals.push(tk);
                else if (c == '(') ops.push(c);
                else if (c == ')') {
                    while (ops.peek() != '(') genCode();
                    ops.pop();
                } else {
                    while (!ops.isEmpty() && precedence(ops.peek()) >= precedence(c)) genCode();
                    ops.push(c);
                }
            }
            while (!ops.isEmpty()) genCode(); 

            printOutputs();
        } catch (Exception e) {
            System.out.println("Error: Invalid Expression!");
        }
    }

    static void genCode() {
        String op2 = vals.pop(), op1 = vals.pop(), op = String.valueOf(ops.pop());
        String res = "t" + (tempCount++);
        code.add(new String[]{op, op1, op2, res});
        vals.push(res);
    }

    static int precedence(char c) {
        return (c == '*' || c == '/') ? 2 : (c == '+' || c == '-') ? 1 : 0;
    }

    static void printOutputs() {
        System.out.println("\n=== Three-Address Code ===");
        for (String[] i : code) System.out.println(i[3] + " = " + i[1] + " " + i[0] + " " + i[2]);

        System.out.println("\n=== Quadruples ===");
        System.out.printf("%-5s | %-5s | %-5s | %-5s\n", "Op", "Arg1", "Arg2", "Res");
        System.out.println("---------------------------------");
        for (String[] i : code) System.out.printf("%-5s | %-5s | %-5s | %-5s\n", i[0], i[1], i[2], i[3]);

        System.out.println("\n=== Triples ===");
        System.out.printf("%-4s | %-5s | %-5s | %-5s\n", "Id", "Op", "Arg1", "Arg2");
        System.out.println("---------------------------------");
        Map<String, String> tMap = new HashMap<>(); // Maps temps to triple indices
        for (int j = 0; j < code.size(); j++) {
            String[] i = code.get(j);
            String a1 = tMap.getOrDefault(i[1], i[1]); // Replace 't1' with '(0)'
            String a2 = tMap.getOrDefault(i[2], i[2]); 
            System.out.printf("(%d)  | %-5s | %-5s | %-5s\n", j, i[0], a1, a2);
            tMap.put(i[3], "(" + j + ")");
        }
    }
}