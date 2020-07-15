 # Illuin
Small server to host images (Stateless). It only relies on filesystem.

## Requierements:
* `cargo`
* To compile it requires `Rust Nightly`

## How to:
### Compile:
`cargo build ` (for production add `--release`)

### Run
#### With cargo (from within the project folder):
`cargo run -- <ARGS ...>`

#### Binary
*It requires to have the static folder next the the binary.*

`./Illuin`

#### Get the options
## To get the help from the CLI
`cargo run -- -h` or `./illiun -h`
