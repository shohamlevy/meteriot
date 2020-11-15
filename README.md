# meteriot
A lighgweight and fast TSDB in rust.
Meters + Umbrellas un Heb.

# Objectives
* Very fast inserts (updates and deletes don't matter).
* Fast queries. But not at expense of inserts!
* Lightweight
* Very high cardinality.
* Scale up, not out. It is intended to be embedded into
projects, not as a standalone product.
* Fixed, pre-defined, and potentially even pre-compiled
schema. It should be custom fit compiled per useage.
* Optimized for bulk inserts.
* A handful of writers, many readers.

# Non objectives
* Storage space efficiency. Yet it should not be a hog.
* Multi protocol, multi purpose.
* Scale out. Only scale up. 
* Replication. Rely on storage resiliency and 3rd
party tools  

# Technincalities
## Rust for high optimization

## GRPC to receive inputs
* Is grpc serialize/deserialize a goid choice?
* Over quic (tcp/3) or simple tcp? Can quic help in case
of big binary files?

## Indexes and tags in sqlite

# Design
