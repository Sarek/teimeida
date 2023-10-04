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

# Installation

## Step 1: Create Directories for Volumes

First, the directories to store the configuration and the uploaded data must be created:

```
$ mkdir config data
```

## Step 2: Set User/Password for Content Upload

The next step is to create the necessary users with their passwords for content uploads. The format is to supply
a username followed by a colon and the password. One such combination can be entered per line. This configuration
is expected in `config/users.conf`:

```
$ echo "user:secret" > config/users.conf
```

## Step 3: Execute the Docker Container

The TeiMeiDa Docker COntainer can be downloaded from the GitHub Container Registry. We want to mount the `config` and `data` directories created earlier into the container, and expose port 8080, on which TeiMeiDa is listening:

```
$ docker run -v ./data:/data -v ./config:/config -p 8080:8080 ghcr.io/sarek/teimeida:latest
```

## Step 4: Open the Index Page

You can now open the index page at [localhost:8080](http://localhost:8080), with links to upload new data, and also to view the overview of already present data.
