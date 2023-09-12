# notes while building

~~I'm starting to think every triple needs a datetime :|

Resist internal predicates for datetime and type - make the API honest about
values being strings and "as_int" your way with rust idioms only.~~

On the other hand,

Nothing stopping the support of both use cases, one where subject is an entry in 
a time series of clumps of observations and another if subject is a thing.  in
the former, the k8p internal predicate works.  in the later, well, not all
information modeling is in ts.

## features

sparql queries - use lalrpop as a language translator.  give a set of sparql
and sql queries as before and after and see if llms can make a grammar and
AST API.
