// A compact source file (Java 25): the guest runs its main, as `java Hello.java` would.
void main() {
    IO.println("Hello from Java " + Runtime.version().feature() + " on Hyperlight!");
}
