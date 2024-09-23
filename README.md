## Quickserving CLI

CLI wrapper around the quickserving-core module.

### Installation

1. Install prerequisites cargo and rustc from [official rust website](http://rust-lang.org)
2. Clone the repository

```bash
git clone thishttps://github.com/Baguuc/quickserving-cli.git
```

3. Build the executable

```bash
cd path_to_cloned_repo
cargo build --release
```

4. Copy the executable target/release/quickserving to your systems applications path specified in enviroment variables.

### How to use

1. cd into directory you have your site's files in.

```bash
cd my_site_directory
```

2. Run the server

```bash
quickserving
```

### CLI Arguments

quickserving-cli have plenty of cli arguments to customize how the server is setup.

- --port <PORT> The port that server will be listening for requests on. [default: 8080]

#

- --directory <DIRECTORY> The directoryectory that will be served. [default: .]

#

- --index <INDEX> The file that will be read from requested path when user requests url ending with '/'. [default: index.html]

#

- --not-found <NOT_FOUND> The file that will be served when the file requested by user is not avaible. [default: 404.html]

#

- -h, --help List all the arguments avaible

#

- -V, --version See the installed version of Quickserving
