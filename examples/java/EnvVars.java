// The host's environment variables: hluk run --env MY_VAR=hello_world ...
void main() {
    for (String key : List.of("MY_VAR", "DEBUG", "GREETING"))
        IO.println(key + "=" + Objects.requireNonNullElse(System.getenv(key), ""));
}
