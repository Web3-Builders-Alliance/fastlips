# Fastlips
A multi-threaded, rust-accelerated image generator based upon Hashlips metadata and layers.

# Usage
Fastlips itself is actually a library. If you would like to use it as a CLI tool, you can use fastlips-cli.

You will need to provide 3 arguments (or match their default settings):

1. Layer path (defaults to /input/layers)
2. Output path (defaults to /output)
3. Layer path (defaults to /input/metadata.json)

You can run the tests in lib.rs to see how it works in greater detail.