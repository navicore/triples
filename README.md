# triples

Experimental Rust lib to store any data in triple format.

## Features

* embedded
* async
* cli db maintenance tool
* RDF / Turtle
* import / export of non-RDF data

## Overview

* Rust API
  * see [crate](https://crates.io/crates/triples)
* stores subject, predicate, and object where
  * subject is always an RDF name
  * predicate is always an RDF name
  * object is always a UTF string

## Install

```bash
cargo install triples
```
or

see [crate](https://crates.io/crates/triples)

## Usage

For API usage, see the unit tests in [db_api]("./src/db_api.rs") for now.

For cli usage:

```bash
triples -h
```

```bash
A lib and cli for storing data triples

Usage: triples [OPTIONS] <COMMAND>

Commands:
  import-turtle
  export-turtle
  import-csv
  export-csv
  help           Print this message or the help of the given subcommand(s)

Options:
  -d, --db-location <DB_LOCATION>  [default: /tmp/triples.db]
  -h, --help                       Print help
  -V, --version                    Print version
```

## TODO

* ~~bulk loading and exporting via cli~~
* ~~normalizes RDF names~~
* ~~normalizing object values~~
* ~~import / export of RDF Turtle `*.tll` format~~
* ~~import / export of csv `*.csv` format~~
* txn control via api
* insert performance
* SparkQL

----------
__PRs welcome__
