# TeiMeiDa

A web tool where you can upload large files and set an expiration date. A link
will be automatically created for you to share via, e.g., e-mail. After the
expiration date is reached, the data will be automatically removed.

## What is special about this?

### No Database

TeiMeiDa does not use any database to save the meta-data. Instead, everything
related to the uploads is stored as [extended attributes](https://en.wikipedia.org/wiki/Extended_file_attributes).

### Rust

TeiMeiDa is written in Rust. It is actually my first "real" Rust project, which
hopefully explains some non-optimal choices I made.

### Smallest Possible  Docker Container

TeiMeiDa is intended to be run as a Docker Container. In contrast to many other
applications, I wanted to provide an image that is as small as possible. Thanks
to Rust's static linking, and linking against the [musl C library](https://musl.libc.org/)
it was possible to make the container only contain the compiled binary, the
configuration file and the necessary directories.
