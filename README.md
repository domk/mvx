# MVX

## Purpose

The `mvx` command aims at providing a `mv`/`cp`  tool modifying only file extension
while keeping the basename constant and preventing such kind of mistake:

```shell
$ ls -1
light.md
$ mv light.md ligt.md.bak
$ ls -1
ligt.md.bak
```

## Examples

To perform the same kind of change, `mvx` can be invoked with:

```shell
$ mvx light.md -a .bak
$ ls -1
light.md.bak
```

`mvx` allows also to make a copy instead of a move using the `--copy` switch:

```shell
$ mvx -c light.md -a .bak
$ ls
light.md
light.md.bak
```

One can also remove an extension with the `--remove` argument:

```shell
$ ls
light.md.bak
$ mvx light.md.bak -r .bak
$ ls
light.md
```

## Installation

Copy the repository locally, then:

``` shell
$ cargo build --release
$ cp target/release/mvx /wherever/you/want
```

If  `cpx` is a link to `mvx` name, the program will perform as if the `--copy`  flag was provided:

```shell
$ cp target/release/mvx /wherever/you/want/cpx
$ ls
light.md.bak
$ /wherever/you/want/cpx light.md.bak -r .bak
$ ls -1
light.md
light.md.bak
```

