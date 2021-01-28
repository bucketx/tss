# TSS

TSS is an application that scales images to match the requirements of Telegram stickers.

## Installation
The executables for common platforms (currently Linux, Windows and macOS) are uploaded after building in GitHub Actions. 
You may select a workflow run and the corresponding platform you need [here](https://github.com/bucketx/tss/actions) to download the pre-built executables.

## Build from source
If you don't want to use pre-built executables or you have made changes to the source code, you may build the application from source yourself.

Requirements:
- Cargo (Rust's official build tool and package manager)
- Rust toolchain (You may install it through Rustup with the instructions [here](https://www.rust-lang.org/learn/get-started))

Step 1: Clone the source code
```base
git clone https://github.com/bucketx/tss.git
```

Step 2: Build
```base
# run this if you want to build an executable with debug information
cargo build

# run this if you want to build an release executable
cargo build --release
```

Your executable will be produced in "[project path]/target/debug" or "[project path]/target/release" depending on what build profile you have used.

## Usage
```bash
tss -i <image>... -o <output directory>
```
TSS will scales the image(s) inputted in -i flag and output the results into the directory inputted in -o flag.

## License
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)
