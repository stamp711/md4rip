# md4rip

A tool for creating MD4 collision.

[Cryptanalysis of the Hash Functions MD4 and RIPEMD](https://doi.org/10.1007/11426639_1)

[![asciicast](https://asciinema.org/a/237747.png)](https://asciinema.org/a/237747)

## `md4rip` usage

```bash
# Given a file <INPUT>, generate MD4 collision starting at byte offset <OFFSET>, write output to <OUTPUT1> and <OUTPUT2>
md4rip <INPUT> <OFFSET> <OUTPUT1> <OUTPUT2>
# Use -j to generate specific pattern (0x__01FEFF) for JPEG COMMENT
md4rip -j <INPUT> <OFFSET> <OUTPUT1> <OUTPUT2>
```

## `md4` usage

```bash
# Given a file's <PATH>, calculate its MD4 digest
md4 <PATH>
```
