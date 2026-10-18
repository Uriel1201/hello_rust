// Name: Dante1201 (Carlos Uriel García)
// Id:   1201

// Write a program arrays_thue_morse.rs that takes 
// an integer command-line argument n and prints an n-by-n pattern. 
// Include two space characters between each + and - character.
 
// ~/hello-rust> javac -d out introcs/arrays/ThueMorse.java
 
// ~/hello-rust> java -cp out ThueMorse 8

public class ThueMorse {
    
    public static void main(String[] args) {
        
        int n = Integer.parseInt(args[0]);
        if (n >= 0) {
            int[] a = new int[n];
            
            for (int i = 1; i < n; i++) {
                if (i % 2 == 0) a[i] = a[i / 2];
                else a[i] = 1 - a[i - 1];
            }
            for (int i = 0; i < n; i++) {
                for (int j = 0; j < n; j++) {
                    if (a[i] == a[j])
                        System.out.print("+  ");
                    else System.out.print("-  ");
                }
                System.out.println();
            }
        }
        else {
            System.out.println("n must be non-negative");
        }
    }
}
