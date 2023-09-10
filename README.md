# triples

Experimental Rust lib that will store any data in triple format.

# UNDER CONSTRUCTION

## Features

* main API / usage is embedded - ie via rust API and crate dependency via cargo.toml
* stores only subject, predicate, and object where 
  * subject is always a UUID
  * predicate is always an RDF name
  * object is always a UTF string
* append-only / intended for event-sourcing
* supports numeric as well as feature / categorical values
* bulk loading and exporting via cli
* maintains verbose RDF names in a separate table for query efficiency for large datasets and storage efficiency
* import / export of RDF Turtle `*.tll` format
* eventual support of SparkQL queries
