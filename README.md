# Spine Atlas Cutter

This application cuts up a Spine (Spine2D animation framework) Atlas texture by a given path to the atlas description file (usually ends with `.atlas`) and the combined image (or atlas) file (usually ends with `.png`).

## Installation

There are 2 main ways of installing this application: using the Releases section and building manually.

### Grabbing a Release binary

Head on over to the [Releases section](https://github.com/catink123/spine-atlas-cutter/releases) and download the latest application binary. At the time of this writing, only Windows binaries are available.

### Building manually

First, install the Rust compiler and the Cargo package manager. This can be done using the [rustup](https://rustup.rs) tool. Then, clone (download) this repository and open up a terminal window in the repo's directory and execute the following command:

```
$ cargo install --path .
```

This will build the project in the Release mode and install it to a standard Cargo binary directory, which can be added to path.

## Usage

To cut up an Atlas texture, you need to have the Atlas image itself and the corresponding description file. Once you get a hold of them, open up a terminal window and type the following command:

```
$ spine-atlas-cutter --image <atlas-image-path> --output-dir <output-dir-path> --atlas <atlas-description-path>
```

The `output-dir` is the destination directory of the cut up image files.

If you have any issues with this application, feel free to write to the [Issues section](https://github.com/catink123/spine-atlas-cutter/issues).