# triples

Experimental Rust lib that will store any data in triple format.

# UNDER CONSTRUCTION

## Features

* stores only subject, predicate, and object where subject is always a UUID and predicate is always an RDF name
* append-only / intended for event-sourcing
* stores all values as string / utf so good for numeric as well as feature / catagorical values
* bulk loading and exporting via cli
* maintains verbose RDF names in a separate table for query efficiency for large datasets and storage efficiency
* import / export of RDF Turtle tll format

Primarily a lib to be imported as a dependency, it will also have a cli to
perform DB maint such as removing tombstones and compaction.
