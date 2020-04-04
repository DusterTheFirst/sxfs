# ShareX File Server
![GitHub](https://img.shields.io/github/license/dusterthefirst/sxfs)
![GitHub issues](https://img.shields.io/github/issues/dusterthefirst/sxfs)
![GitHub pull requests](https://img.shields.io/github/issues-pr/dusterthefirst/sxfs)
![GitHub last commit](https://img.shields.io/github/last-commit/dusterthefirst/sxfs)
![GitHub Release Date](https://img.shields.io/github/release-date/dusterthefirst/sxfs)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/dusterthefirst/sxfs)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/dusterthefirst/sxfs/Build)
![Docker Image Size (latest by date)](https://img.shields.io/docker/image-size/dusterthefirst/sxfs)
![MicroBadger Layers](https://img.shields.io/microbadger/layers/dusterthefirst/sxfs)
![Docker Pulls](https://img.shields.io/docker/pulls/dusterthefirst/sxfs)

A single binary file server for handling uploads from the [ShareX client] with a web ui written
in rust and containerized with docker for your pleasure

# Usage
There are two provided ways of running sxfs, either through docker (preferred) or using the binary directly.

## Binary
You can download one of the provided binaries that can be found from the [latest successful actions run].
If you need a binary for a platform not provided, you can build the program from source. Information for building can be found in the [building section](#Building).

<details>
    <summary>Command line usage</summary>
    sxfs 0.1.0
    A file server for handling uploads from the ShareX client

    USAGE:
        sxfs.exe [FLAGS] [OPTIONS]

    FLAGS:
        -h, --help          Prints help information
        -r, --rocket-log    Enable rocket info logging (requires info logging)
        -V, --version       Prints version information
        -v, --verbose       Enable verbose logging. (1 = informational, 2 = debug, 3 = trace)

    OPTIONS:
        -a, --address <address>        The address to bind to [default: 0.0.0.0]
        -c, --config <config>          The path to the config file [default: data/config.toml]
        -d, --database <database>      The path to the sqlite database that holds the mappings between uploads and their files aswell as [default: data/db.sqlite]
        -p, --port <port>              The port to bind to [default: 8000]
        -u, --uploaders <uploaders>    The path to output the generated ShareX custom uploaders file [default: data/uploaders]
</details>

## Docker


## Setting up HTTPS
sxfs by default does not provide https support for safety reasons.
In order to use https you must either put it behind your own HTTPS secured reverse proxy
or you could use a service like cloudflare to secure your server.

## Adding the uploader
To get your custom uploader/shortener you can either download them through the web panel,
using the downloads links on the header, or directly access them from the data directory

## Repurposing for use with a custom upload client
Although there is ShareX in the name of the program, it is not restricted to use only by
the ShareX client. It can be used by any file uploader/url shortener that supports custom
endpoints.

To configure uploads, `POST /u` with the body of the request as the
file contents, a GET parameter `filename` equal to the origional filename to upload and the header
`X-Upload-Token` set to the token found in your config. The server will respond with JSON data
containing the `filename` of the uploaded resource aswell as the generated upload `id`.

To configure link shortening, `POST /l` with a GET parameter `uri` set to the URI to shorten
and authenticate the request with the `X-Upload-Token` set to the token found in your config.
The server will respond with JSON data containing the generated link `id`

# Building
Requires `cargo` (comes with [Rustup]) and `yarn` ([Yarn Website]).
```sh
$ git clone https://github.com/DusterTheFirst/sxfs # Clone the repository
$ yarn global install typescript # Needed to build the web client
$ cargo build --release # To produce the binary in ./target/release/sxfs
$ cp ./target/release/sxfs /usr/bin # To be able to use the program from the command line
$ sxfs # Start the program
```

# License
    A file server for handling uploads from the ShareX client with a web ui
    Copyright (C) 2020  Zachary Kohnen

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

[ShareX client]: https://getsharex.com/
[Rustup]: https://rustup.rs/
[Yarn Website]: https://yarnpkg.com/
[latest successful actions run]: https://github.com/DusterTheFirst/sxfs/actions
[building section]: #Building