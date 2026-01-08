# Hex2string

*Hex2string* is a simple command line tool that converts hexadecimal strings to normal strings.

## Installation

### Unix users (Linux, BSDs and MacOSX)

Unix users may download and install latest *hex2string* release with command:

```bash
sh -c "$(curl https://sweetohm.net/dist/hex2string/install)"
```

If *curl* is not installed on you system, you might run:

```bash
sh -c "$(wget -O - https://sweetohm.net/dist/hex2string/install)"
```

**Note:** Some directories are protected, even as *root*, on **MacOSX** (since *El Capitan* release), thus you can't install *hex2string* in */usr/bin* for instance.

### Binary package

Otherwise, you can download latest binary archive at <https://github.com/c4s4/hex2string/releases>. Unzip the archive, put the binary of your platform somewhere in your *PATH* and rename it *hex2string*.

## Usage

To convert a hexadecimal string to a normal string, run:

```bash
hex2string <hex>
```

*Enjoy!*
