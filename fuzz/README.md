# Fuzzing the host functions

A guest is untrusted, and so is everything it sends the host. A program in the guest goes through the kernel, which hands the host clean arguments, but native code in the guest can make any host call itself, with any arguments. The targets here call the host functions a guest reaches with what such code can send, and check that the host answers every call well, refuses what the sandbox does not allow, and neither crashes, hangs nor leaks.

They use [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) (libFuzzer), which needs a nightly toolchain: `just fuzz` installs the one the justfile pins, and cargo-fuzz, when missing. Linux and macOS.

```bash
just fuzz fuzz_hostfs                       # until an input fails, or Ctrl-C
just fuzz fuzz_hostnet -max_total_time=60   # libFuzzer's arguments follow the target
just fuzz-build                             # build every target
```

The targets are built with debug assertions, and so with overflow checks. A target starts from its corpus in `fuzz/corpus/<target>/` and adds to it the inputs that reach new code. An input that fails is saved in `fuzz/artifacts/<target>/`: run the target on it alone to see the failure again, and print it as the calls it makes:

```bash
cargo +nightly-2026-09-22 fuzz run --debug-assertions fuzz_hostfs fuzz/artifacts/fuzz_hostfs/crash-<hash>
cargo +nightly-2026-09-22 fuzz fmt --debug-assertions fuzz_hostfs fuzz/artifacts/fuzz_hostfs/crash-<hash>
```

[`fuzz.yml`](../.github/workflows/fuzz.yml) runs every target weekly, and on pull requests that change them or the code they fuzz (see [`docs/ci.md`](../docs/ci.md)).

## How a call is made

A guest's host call reaches Hyperlight as a FlatBuffer, which hyperlight-common decodes into a function name and a list of parameter values ([Hyperlight's own fuzz targets](https://github.com/hyperlight-dev/hyperlight/tree/main/fuzz) fuzz that decoding). Hyperlight then finds the function by name and converts the list to its parameter types, refusing a list of the wrong length or types before the function runs. The targets start from that list: the library's `fuzzing` module (behind the `fuzzing` feature, and not a stable API) registers the host functions as a sandbox does and calls them as Hyperlight does, with no VM.

`fuzz_hostfs` and `fuzz_hostnet` also make calls of any shape: a host function, or another name, with any parameter values. Such a call must fail exactly when its arguments are not of the function's types, and when it does not, its reply must be well formed.

## Targets

### `fuzz_hostfs`

The `fs_*` functions on four mounts: a host directory read-write and read-only, and an embedder's `FsBackend` read-write and read-only, whose answers the input scripts, lies included (a read longer than its buffer, a listing too large for a reply, a name that is not one component, any OS error code). The directory is two levels below directories the guest must not reach, each with a secret file, and holds symbolic links out of it. The paths are mostly made of the names in the mount, `.`, `..` and empty components, or any text.

- Every reply is well formed: a status that is `0` or a Linux `-errno`, and the layout the kernel reads (`fs_stat`'s 54 bytes, a listing whose entries add up to the reply and are single components, a read no longer than asked, a link target no longer than `PATH_MAX`), within the size of one host call.
- A read-only mount refuses every write with `EROFS`, and its backend is never asked to write.
- After the run, nothing outside the mount has changed, no read returned a secret, no file in the mount is a hard link to a secret, and none is set-user-ID, set-group-ID or sticky.

### `fuzz_hostnet`

The `net_*` functions under an allow list that lists nothing, and no ports to listen on: the guest may serve on a port the host picks (a bind to port 0), and reach nothing. The input can also act as the guest's clients: connect from the host to a socket of the guest's, send it a datagram, and send, read or hang up on a connection. A TCP listener and a UDP socket on loopback stand for the host's services, and the guest is given their addresses, as well as `::ffff:127.0.0.1`, the metadata address and any text.

- Every reply is well formed: a count, a file descriptor or a Linux `-errno`, packed addresses of the length their family has, `net_recvfrom`'s length matching its data and no more than asked, a `revents` per `net_poll` entry and `POLLNVAL` for a negative fd.
- No TCP connect starts, and no datagram is sent: a send succeeds only on a connection the guest accepted. The services hear nothing.
- A `net_poll` with a timeout other than zero is refused. The run waits as the guest and the host do, `net_poll` with a zero timeout and the host's wait between entries with none, so an input that takes long is a call that blocks: CI fails one that takes a minute (`-timeout=60`).
- The run leaves no file descriptor open.

The services, and the clients, are on loopback: a run sends nothing past the machine.

### `fuzz_dns`

The DNS messages the policy reads: a guest's query, which `net_send` and `net_sendto` check before it leaves for port 53, and a resolver's answer, from which `net_recvfrom` learns the addresses of the names the policy allows. The policy is a `Filter` that allows the names under `allowed.test`; a message is any bytes, or one laid out as DNS lays it out, with names the filter allows and others, compression pointers and records of any type; and it comes from the policy's resolver, the resolver on another port, or another host.

- A name read from a message is one a resolver can be asked about: at most 253 characters, in labels of printable ASCII.
- A query is allowed exactly when the filter allows every name it asks.
- An answer from the resolver, for a name the filter allows, makes its addresses reachable; one from anywhere else, or for another name, makes none. No answer makes loopback, the unspecified address or a metadata address reachable.
