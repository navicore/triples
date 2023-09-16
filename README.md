# triples

Experimental Rust lib to store any data in triple format.

# UNDER CONSTRUCTION

## Features

* embedded
* async
* cli db maintenance tool
* RDF / Turtle

## Overview

* main API / usage is embedded - ie rust API and crate dependency
  * see [crate](https://crates.io/crates/triples)
* stores only subject, predicate, and object where
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

* ~~bulk loading and exporting via cli~~ (done)
* ~~normalizes RDF names~~ (done)
* considering the normalized object values
* ~~import / export of RDF Turtle `*.tll` format~~ (done)
* import / export of csv `*.csv` format
* eventual support of SparkQL queries
