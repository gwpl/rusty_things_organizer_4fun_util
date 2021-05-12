
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

# register containers
$ things c
container00
container01
container02
...

# move to container multiple items
$ things m container_code
thing_code01
thing_code02
thing_code03

# batch mode. Empty line indicates start of new sequence.
# multiple empty lines in a row are allowed.
# Beginning of each sequence if container, later is followed by items inside.
$ things b
container00
item00
item01

container10
item10
item11




container20
item20
...


# list things in container
$ things l container_code

# list containers
$ things lc
```


