# mcrputil

![license](https://img.shields.io/badge/License-Apache_2.0-blue.svg)
![version](https://img.shields.io/badge/Version-1.0.0-green.svg)

Minecraft Resource Pack Util for encrypting, decrypting, signing and verifing resource packs. 

## Usage

```
Usage: mcrputil <COMMAND>

Commands:
  encrypt  Encrypts the folder with a given or auto-generated key
  decrypt  Decrypts the folder with a given key
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

### Encryption
```
Usage: mcrputil encrypt [OPTIONS] --id <ID> <INPUT> <OUTPUT> [KEY]

Arguments:
  <INPUT>   Input file or folder
  <OUTPUT>  Output folder

Options:
  -i, --id <ID>            Content id (Resource pack uuid)
  -k, --key <KEY>          Key used for encryption
  -e, --exclude <EXCLUDE>  Specifies files which should not be encrypted
  -h, --help               Print help information
```

### Decryption
```
Usage: mcrputil decrypt [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input file or folder
  <OUTPUT>  Output folder

Options:
  -k, --key <KEY>  Key used for decryption
  -h, --help       Print help information
```
