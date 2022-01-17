## Instaget

A Command line tool for downloading video or image data from `Instagram`

[![instaget CI](https://github.com/wuriyanto48/instaget/actions/workflows/ci.yml/badge.svg)](https://github.com/wuriyanto48/instaget/actions/workflows/ci.yml)

# 

### Install

MacOS
```shell
$ wget http://storage.teknologipedia.id/instaget/osx/instaget
$ chmod +x instaget
$ ./instaget -h
```

Linux
```shell
$ wget http://storage.teknologipedia.id/instaget/linux/instaget
$ chmod +x instaget
$ ./instaget -h
```

# 

### If you prefer to build your own binary from source
Requirements
- Rust https://www.rust-lang.org/

Build
```shell
$ git clone https://github.com/wuriyanto48/instaget.git
$ cd instaget/
$ make build
```

# 

### Download `photo` or `video` with simple command line tools

show help
```shell
$ ./instaget -h
```

download from shared URL
```shell
$ ./instaget https://www.instagram.com/p/CYjktDFqxIm/?utm_source=ig_web_copy_link
```

or just the original URL
```shell
$ ./instaget https://www.instagram.com/p/CYjktDFqxIm/
```