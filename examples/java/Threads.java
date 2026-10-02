// Concurrency: an executor's futures, and explicit threads with a lock.
void main() throws Exception {
    try (var pool = Executors.newFixedThreadPool(4)) {
        var futures = new ArrayList<Future<Integer>>();
        for (int i = 0; i < 8; i++) {
            int n = i;
            futures.add(pool.submit(() -> n * n));
        }
        int sum = 0;
        for (var f : futures)
            sum += f.get();
        IO.println("sum-of-squares: " + sum); // 0+1+4+9+16+25+36+49 = 140
    }

    var counter = new int[1];
    var threads = new ArrayList<Thread>();
    for (int i = 0; i < 4; i++) {
        var t = Thread.ofPlatform().start(() -> {
            synchronized (counter) {
                counter[0]++;
            }
        });
        threads.add(t);
    }
    for (var t : threads)
        t.join();
    IO.println("thread-counter: " + counter[0]);

    var virtual = Thread.ofVirtual().start(() -> IO.println("virtual thread ran"));
    virtual.join();
    IO.println("threads-done");
}
