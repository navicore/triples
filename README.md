# triples

Experimental Rust lib to store any data in triple format.

# UNDER CONSTRUCTION

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

## TODO

* ~~bulk loading and exporting via cli~~
* ~~normalizes RDF names~~
* considering the normalized object values
* ~~import / export of RDF Turtle `*.tll` format~~
* ~~import / export of csv `*.csv` format~~
* support of SparkQL queries

----------
__PRs welcome__
