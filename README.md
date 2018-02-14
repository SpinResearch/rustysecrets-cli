
# RustySecrets CLI

[![Build Status](https://travis-ci.org/SpinResearch/rustysecrets-cli.svg?branch=master&style=flat)](https://travis-ci.org/SpinResearch/rustysecrets-cli)
[![Crates.io](https://img.shields.io/crates/v/rustysecrets-cli.svg)](https://crates.io/crates/rustysecrets-cli)
[![license](https://img.shields.io/github/license/SpinResearch/rustysecrets-cli.svg)](License)

> *rustysecrets-cli* is a command-line wrapper around [RustySecrets](https://github.com/SpinResearch/RustySecrets), a Rust implementation of threshold [Shamir's secret sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing).

## Installation

    $ cargo install rustysecrets-cli

## Usage

```bash
$ mkdir shares
$ cat > secret.txt
These programs were never about terrorism: they’re about economic spying,
social control, and diplomatic manipulation. They’re about power.
^D

$ rustysecrets split secret.txt -o shares -k 7 -n 10 -m text/plain
$ ls shares/
share_0 share_1 share_2 share_3 share_4 share_5 share_6 share_7 share_8 share_9

$ rustysecrets recover shares/share_{0-6}
     info: Version: INITIAL_RELEASE
     info: MIME-Type: text/plain
These programs were never about terrorism: they’re about economic spying,
social control, and diplomatic manipulation. They’re about power.

$ rustysecrets recover shares/share_{0-2}
    error: Could not recover secret
caused by: Not enough shares provided!
```

## Documentation

### `rustysecrets`

```
USAGE:
    rustysecrets <SUBCOMMAND>

OPTIONS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    recover    Recover the secret from the shares [aliases: r]
    split      Split a secret into shares [aliases: s]
```

### `rustysecrets split`

> Split a secret into shares

```
USAGE:
    rustysecrets split [OPTIONS] <INPUT> --output <DIR> -k <k> -n <n>

OPTIONS:
    -o, --output <DIR>               Path to the directory to output the shares to
    -m, --mime <MIME>                The MIME type of the secret
    -h, --help                       Prints help information
    -k <k>                           Number of shares necessary to recover the secret
    -n <n>                           Total number of generated shares
    -r, --raw                        Include this flag to generate raw shares (ie. without a MIME type)
    -t, --share-tmpl <share-tmpl>    Template for the share names. Defaults to 'share_{{num}}'
    -s, --sign                       Sign the shares
    -v, --verbose                    Enable verbose mode

ARGS:
    <INPUT>    Path to the file containing the secret to split, or - to read from stdin
```

### `rustysecrets recover`

> Recover the secret from the shares

```
USAGE:
    rustysecrets recover [OPTIONS] <SHARES>...

OPTIONS:
    -o, --output <FILE>    Path to file to output the secret to, prints to stdout if omitted
    -h, --help             Prints help information
    -r, --raw              Include this flag if the shares are raw (ie. do not contain a MIME type)
    -v, --verbose          Enable verbose mode
        --verify           Verify the shares signatures

ARGS:
    <SHARES>...    Paths to shares to recover the secret from
```

## Bug Reporting

Please report bugs either as pull requests or as issues in [the issue
tracker](https://github.com/SpinResearch/rustysecrets-cli). *rustysecrets-cli* has a
**full disclosure** vulnerability policy. **Please do NOT attempt to report
any security vulnerability in this code privately to anybody.**

## License

RustySecrets CLI is released under the BSD3 license. See [LICENSE](LICENSE) for more informations.

