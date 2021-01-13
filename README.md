# IsRunning

IsRunning is a very simple and lightweight command-line utility to check if some processes are running or not.

It expects a list of process's names as argument and simply exits with code `0` if they were found, or `1` otherwise.
No output or message is printed.

Usage:
```
is_running PROCESS_1 [PROCESS_2] [PROCESS_N]
```

_Examples_
```
$ ./is_running nginx
$ echo $?
0
```

```
$ ./is_running nginx node
$ echo $?
0
```

```
$ ./is_running dead
$ echo $?
1
```

### Motivation
Its main purpose is to serve as readiness probe for containers that spawn multiple processes.

For example, a [Pod readiness probe](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/) to detect if `nginx` is running:
```
    readinessProbe:
      exec:
        command:
        - /usr/bin/is_running
        - nginx
      failureThreshold: 3
      periodSeconds: 5
      successThreshold: 1
      timeoutSeconds: 5
```

> It's not advisable to run multiple processes on a single container, but for many reasons, it's not uncommon, especially for legacy applications and Microsoft Windows containers.

### Why not use a bash script?
I used to use a bash script combining `ps` and `grep` together:

```
$ ps | grep nginx
$ echo $?
0
```

It works, but it's not the most efficient solution, particularly when you call it every 5 seconds for hundreds of containers.

This utility saves a little bit of CPU and memory by removing the overhead of `bash` interpreter and all the string processing between `ps` and `grep`.

### Further improvements
This utility is implemented in Rust, just because we needed a native, small, fast, and secure binary to perform such task.
It uses the [sysinfo](https://docs.rs/sysinfo/0.15.8/sysinfo/) crate, which provides a nice abstraction for querying OS's processes.

Please, feel free to open an issue or send a PR if you might improve this implementation in some sense.

