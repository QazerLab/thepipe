Table Header Excluded Pipe
==========================

When you pipe stdout of some command to another one, every line
is fed to the latter. If the output of the first one is a table,
its first line (the header) is processed by the second command
along with the table content, which is frequently not the desired
behavior.

Table Header Excluded Pipe (`thepipe`) echoes the first line of
stdin received from somewhere as is and pipes the rest of the input
to the child command, as the usual pipe does, for example:

```
$ ps | thepipe sed 's!pts/!!'
    PID TTY          TIME CMD
  14107 5    00:00:00 bash
  14128 5    00:00:00 ps
  14129 5    00:00:00 thepipe
  14130 5    00:00:00 thepipe
```

The table header stays untouched while the table content is processed
through `sed`.

Given that the most frequent use of this tool is filtering the content
of the table with `grep`, there is a wrapper `thegrep`, which does exactly
what `thepipe grep` would do:

```
$ docker images | thegrep postgres
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
postgres            latest              c96f8b6bc0d9        2 weeks ago         314MB
```


Build
-----

To build `thepipe` you need a [Rust toolchain](https://www.rust-lang.org/tools/install).

Once you have it on your machine, you can build `thepipe` by running

```
cargo build --release
```

The resulting binary is `target/release/thepipe`.

To install it into your system, run

```
sudo ./install
```

This will place `thepipe` and `thegrep` into your `/usr/bin`.
