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

import csv and prepend NS prefixes

```bash
cat ../vssgen/vss_sm.csv | triples -d /tmp/vss.db import-csv --subject-default-ns https://myvss.com/id --predicate-default-ns https://myvss.com/data --skip-headers
```

export ttl

```bash
triples --db-location /tmp/vss.db export-turtle
```

```bash
@prefix ns1: <https://myvss.com/id/> .

@prefix ns2: <https://myvss.com/data/> .

ns1:d654c9bc-37d7-425e-945b-41a4440da236
    ns2:has_chassis "1ad84bfb-2017-4c42-b28f-de938755cb00" ;
    ns2:timestamp "2023-09-17 21:07:36" ;
    ns2:type "vehicle" ;
    ns2:has_drivetrain "8db9fa98-5017-43d3-accd-bebe822a4066" ;
    ns2:vehicle_id "0" ; .

ns1:1ad84bfb-2017-4c42-b28f-de938755cb00
    ns2:brake_status "False" ;
    ns2:type "chassis" ;
    ns2:speed "79" ; .

ns1:8db9fa98-5017-43d3-accd-bebe822a4066
    ns2:engine_temperature "106" ;
    ns2:fuel_level "56" ;
    ns2:type "drivetrain" ;
    ns2:battery_level "3" ; .
```

## TODO

* ~~bulk loading and exporting via cli~~
* ~~normalizes RDF names~~
* ~~normalizing object values~~
* ~~import / export of RDF Turtle `*.tll` format~~
* ~~import / export of triple csv `*.csv` format~~
* meaningful prefix names on export
* import of arbitrary column csv `*.csv` format
* import of arbitrary json `*.json` format
* import of arbitrary jsonl `*.jsonl` format
* txn control via api
* insert performance
* SparkQL

----------
__PRs welcome__
