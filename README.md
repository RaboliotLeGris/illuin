 # Illuin
Small server to host images (Stateless). It only relies on filesystem.

## How to use

By default Illuin listens on `8080` and store data in the temporary data folder (ex: on Linux `/tmp/illuin`; on Windows `C:\Users\YOURUSER\AppData\Local\Temp\illuin`)

Illuin is not entirely configuration through the Env vars, it only listens for the port with the key `PORT`.

### CLI parameters

* `--base-route <base_route>`: Prepend all routes with provided value. Format must be like: `/base/` [default: /]
* `--port <port>`, `-p <port>`, : Use a the listen port of the app [default: 8080]
* `--storage-path <storage_path>`, `-s <storage_path>`: Path where the image are stored [default: temporary folder of your OS]
* `-h`: display the help information

### Practical use

To properly use Illuin (in a docker for instance), you should have a dedicated folder.

Example: `./illuin --storage-path /srv/illuin/data`

## How to build
## Requirements:
* `Rust` toolchain with `nightly`
* To compile it requires `Rust Nightly`

### How to compile:
`make build` or `make build-release` (for production) or `cargo build` or `cargo build --release` (for production)

### Run
#### With cargo (from within the project folder):
`cargo run -- <ARGS ...>` (`--` is mandatory, it delimits cargo args and the app args)

#### Binary
*It requires to have the template folder next the the binary.*

*Once compiled the binary is in the target/[debug or release]/illuin*

`./Illuin`

### Test
* `make CI` or `cargo check && cargo clippy && cargo test`
