# Java runtime for Hyperlight/Unikraft.
#
# The guest runs Java source in-process with JShell: the host sends source
# and output comes back, same dispatch model as Python/Node and the
# dotnet-jit image.  javac runs in the guest; no host-side compilation.
#
# A jlinked Java 25 runtime on Alpine (musl) keeps the rootfs
# self-contained with its own dynamic linker.  A CDS archive of the
# classes the dispatcher loads while it warms up (javac's included)
# starts the JVM from mapped, pre-parsed classes.

FROM eclipse-temurin:25-jdk-alpine AS build

RUN apk add --no-cache gcc musl-dev binutils

# The runtime: what JShell needs, and the modules code commonly imports
# beyond java.base (HTTP client and server, SQL, XML, JNDI, logging,
# management beans).
# The rootfs is unpacked into guest memory, so it is kept small: the
# modules compressed (most classes load from the CDS archives anyway), no
# archive for a heap without compressed pointers, which a guest's never
# is, and the JVM without its symbols.
RUN jlink --add-modules java.base,java.compiler,java.logging,java.management,java.naming,java.net.http,java.sql,java.xml,jdk.compiler,jdk.httpserver,jdk.jshell,jdk.unsupported,jdk.zipfs \
        --strip-debug --no-man-pages --no-header-files --generate-cds-archive \
        --compress=zip-6 --output /opt/java-rt && \
    rm /opt/java-rt/lib/server/classes_nocoops.jsa && \
    strip --strip-unneeded /opt/java-rt/lib/server/libjvm.so

# The dispatch server and the hyperlight package user code sees.
COPY drivers/java/HlJavaDispatch/src/ /src/java/
RUN mkdir -p /classes /app && \
    javac --release 25 -d /classes $(find /src/java -name '*.java') && \
    printf 'Add-Opens: java.base/java.io java.base/java.lang jdk.jshell/jdk.jshell\n' > /manifest && \
    jar --create --file /app/hl-java.jar --main-class hyperlight.Dispatch \
        --manifest /manifest -C /classes .

# Compile the C driver
# Preserve directory layout so #include "../hl_fc.h" resolves
COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/hl_child.h /src/drivers/hl_child.h
COPY drivers/java/hl_javadriver.c /src/drivers/java/hl_javadriver.c
RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        -static-pie -fPIE \
        -o /hl_javadriver /src/drivers/java/hl_javadriver.c

# The CDS archive: run the dispatcher through its warm-up with the options
# the driver starts it with (an archive is only used under the same ones),
# from the paths it has in the guest (the JDK makes way for the runtime at
# /opt/java), and with a guest's memory: a heap past 32 GiB, from this
# machine's, would turn compressed pointers off, and the archive with them.
# Its call pipe is /dev/null, so it exits once warm, and the JVM writes
# the archive on the way out.
COPY drivers/java/jvm.args /app/jvm.args
RUN mv /opt/java /opt/jdk && mv /opt/java-rt /opt/java && \
    /opt/java/bin/java @/app/jvm.args -XX:MaxRAM=1g -XX:ArchiveClassesAtExit=/app/hl-java.jsa \
        -jar /app/hl-java.jar 3 4 3</dev/null 4>/dev/null && \
    test -s /app/hl-java.jsa

# musl's dynamic linker is named for the architecture (x86_64, aarch64)
RUN mkdir -p /musl && cp /lib/ld-musl-$(uname -m).so.1 /musl/ && \
    rm -rf /tmp/* /tmp/.[!.]*

# Minimal rootfs: JVM + dispatch server + driver + musl
FROM scratch

COPY --from=build /musl/ /lib/
COPY --from=build /opt/java/ /opt/java/
COPY --from=build /app/ /app/
COPY --from=build /hl_javadriver /usr/local/bin/hl_javadriver

# /tmp needed for runtime temp files
COPY --from=build /tmp /tmp
