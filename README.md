# Simple file encryption/decryption utility

XORCrypt is a simple command-line utility which allows you to encrypt files using a password.  
As it's name implies, it uses XOR encryption.

## ⚠️ Warning
```
This utility is not designed to protect sensitive data.
Do NOT use this utility to protect sensitive information as XOR encryption not secure enough for such application.
```
If you're looking for a secure alternative, try the free and open-source [`rclone`](https://rclone.org/) or [Cryptomator](https://cryptomator.org/).

## Dependencies
- [`clap`](https://crates.io/crates/clap)
    - For command-line argument parsing
- [`rpassword`](https://crates.io/crates/rpassword)
    - Allows reading passwords from the command-line
- [`spinoff`](https://crates.io/crates/spinoff)
    - Allows displaying cool spinners.

## Parts
- [`main.rs`](src/main.rs)
    - The main part of the app.
- [`crypto.rs`](src/crypto.rs)
    - Provides functions for encryption/decryption.

## Usage
Simple encrypt, password will be requested from stdin.
```sh
xorcrypt data.txt -o data.txt.encrypted
```

Or you can pass a password as an argument.
```sh
xorcrypt -p superpassword123 data.txt -o data.txt.encrypted
```

Decryption: *(`-p` can also be used here)*
```sh
xorcrypt -r data.txt.encrypted -o data.txt
```

For help use `xorcrypt --help`.

## Building from source
You can simply clone this repo and use `cargo run` to build the app. No additional configuration is needed.

## Compatibility
- ✅ Windows
- ✅ macOS (x86 & ARM)
- ✅ Linux (x86 & ARM)

## Encryption details
XORCrypt reads `N` bytes from the source file in a loop. `N` is the length of the password. Next, each byte from the `N`-sized buffer is XORed with a byte (ASCII code) from the password.

### Example:  
Let's define a password: `code`.  
Let's say the data we're trying to encrypt (in hexadecimal) is `FE ED BE EF`.

```
FE ED BE EF
^^ ^^ ^^ ^^
|  |  |  |- XOR with 'e' (0x65)
|  |  |- XOR with 'd' (0x6E)
|  |- XOR with 'o' (0x6F)
|- XOR with 'c' (0x63)
```
End result is:
```
FE ^ 63 = 9D
ED ^ 6F = 82
BE ^ 6E = D0
EF ^ 65 = 8A
```
The encrypted data is `9D 82 D0 8A`.

## Additional notes
Files encrypted using XORCrypt will start with a "magic byte". This is used to allow XORCrypt to detect whether this file was encrypted or not. This magic byte is defined [here](src/crypto.rs#7).