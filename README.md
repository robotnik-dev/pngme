# Send secret messages via PNG images

## Installation

Head over to the [latest Release](https://github.com/robotnik-dev/pngme/releases/latest) download and extract the binary.

## Usage
```shell
cd path/to/extracted-binary
```
>Depending on your system u need to change the permissions or allow the binary to be executed (Mac)

```shell
chmod +x pngme
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

A simple number
```shell
./pngme encode image.png HaHa 0123456789
```

or a complete sentence (don't forget the quotation marks)
```shell
./pngme encode image.png HaHa "Your message here!"
```

### Decode a message with the same encoder
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

