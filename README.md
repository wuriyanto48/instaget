## Instaget

A Command line tool for downloading video or image data from `Instagram`

### Build binary from source
Requirements
- Rust https://www.rust-lang.org/

# 

Build
```shell
$ cargo build
```

### Download `photo` or `video` with simple command line tools

download from shared URL
```shell
$ ./target/debug/instaget https://www.instagram.com/p/CYjktDFqxIm/?utm_source=ig_web_copy_link
```

or just the original URL
```shell
$ ./target/debug/instaget https://www.instagram.com/p/CYjktDFqxIm/
```