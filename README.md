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
* ~~normalizing object values~~
* ~~import / export of RDF Turtle `*.tll` format~~
* ~~import / export of csv `*.csv` format~~
* txn control via api
* insert performance
* SparkQL

----------
__PRs welcome__
