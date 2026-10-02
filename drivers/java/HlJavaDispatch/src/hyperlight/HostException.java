package hyperlight;

/** A host function returned an error; the message is the host's. */
public class HostException extends RuntimeException {
    public HostException(String message) {
        super(message);
    }
}
