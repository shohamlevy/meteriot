# meteriot Read Interface
The objective of the "read interface" is to provide a standard API to query the
meteriot data.

* The interface should provide a well known API, and common plugins - such as Grafana
data source.
* It should be lazy, only "translating" parts of meteriot data as needed.

## Options
### MariaDB storage engine
MariaDB has better licensing scheme than MySQL, and is not owned by Oracle.
The DB itself is GPLv2, and so would the engine be. But it can be mixed with FLOSS
licensed code (Apache2 license) is such code, without "infecting" it.
Caveats: MySQL runs on OSX, MariaDB does not.
https://dev.mysql.com/doc/internals/en/custom-engine.html
https://grafana.com/docs/grafana/latest/datasources/mysql/
https://www.codeproject.com/Articles/1107279/Writing-a-MySQL-storage-engine-from-scratch

### PostgreSQL
The MIT license is more along the lines of APL2. But there is no storage engines architecture.
So a special hook system would have to be used.

### ElasticSearch
Use Kibana's schema-on-read to bring data into ES by demand. TBD: should we Reindex the data
fetched, as native ES, for faster retrieval the next time?
TODO: can a schema creation fetch data from outside ES?
https://www.elastic.co/blog/schema-on-write-vs-schema-on-read
