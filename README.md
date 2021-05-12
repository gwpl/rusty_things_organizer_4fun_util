
`rusty_things_organizer_4fun_util`

Tool made for fun to play with rust libraries and to organize some things using barcode reader :).

```
Usage:

Assuming:

* alias things=/path/to/binary...
* environment variable THINGS_DB with THINGS_DB directory

# search
things s XXX

# move to container
$ things m container_code thing_code

# move to container multiple items
$ things m container_code
thing_code01
thing_code02
thing_code03
...

# list things in container
$ things l container_code

# list containers
$ things lc
```


