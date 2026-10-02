// A compact source file (Java 25): the guest runs its main, as `java Main.java` would.
// A static method declared here can also be called from an embedder with
// AppSandbox::call("greet", json) once this file has run.
record Event(String name) {}

record Greeting(String greeting) {}

Greeting greet(Event event) {
    return new Greeting("Hello, " + event.name() + "!");
}

void main() {
    IO.println("Hello from {{name}}!");
    IO.println(greet(new Event("Java " + Runtime.version().feature())).greeting() + " Running inside a Hyperlight micro-VM.");
}
