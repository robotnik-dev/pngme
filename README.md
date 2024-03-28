# Send secret messages via PNG images

## Installation

Head over to the [latest Release](https://github.com/robotnik-dev/pngme/releases) and download the binary.

## Usage
```shell
cd path/to/binary
```
### Using --help to get use info
```shell
./pngme --help
```
or for each subcommand
```shell
./pngme encode --help
```

### Encode a message in an image
```shell
./pngme encode image.png HaHa "Your message here!"
```

### Decode a message with the same encoding
```shell
./pngme decode image.png HaHa
```

### Print all available encodings in this image
```shell
./pngme print image.png
```

### Remove an encoding
```shell
./pngme remove image.png HaHa
```

