

# Tuan CLI

A Rust-based command-line interface (CLI) program to manage your environment variables efficiently.

## Features

-   **Minta Command**: Use the `minta` command to ask for help from Tuan.
    -   **Env Subcommand**: Retrieve environment variables from a remote source.

## Usage

### Installation

#### Via Brew

```bash
brew tap wypratama/tap
brew install tuan
```
If you face error after installation please refer FAQ below.
#### Via Cargo
To install via Cargo, the Rust package manager. Ensure that you have Rust installed. Then, run the following command:

```bash
cargo install tuan
```

### Setup

Create a file named tuan.yaml in your root project containing the environment you want to setup, source (which for now is limited to git source), and branch it saved for example:

```yaml
local:
  source: "git@github.com:wypratama/tuan-cli"
  branch: "env"
```

### Commands

-   **minta env**: Retrieve environment variables from a remote source.


    `tuan minta env <remote_source>`

    Replace `<remote_source>` with the desired remote source. example:

    ```
    tuan minta env local
    ```

### FAQ

- Error **Library not loaded: /usr/local/opt/openssl@3/lib/libssl.3.dylib**
	- Please install openssl@3 by running `brew install openssl@3`
-

## Author

Created by Wicaksana Pratama.

For any questions or feedback, please reach out to [wicaksanapratama@gmail.com](mailto:wicaksanapratama@gmail.com).
