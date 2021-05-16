
# `rusty_things_organizer_4fun_util`

Tool made for fun to play with rust libraries and to organize some things using barcode reader :).

Repos:

* [`https://gitlab.com/gwpl/rusty_things_organizer_4fun_util`](https://gitlab.com/gwpl/rusty_things_organizer_4fun_util)
* [`https://github.com/gwpl/rusty_things_organizer_4fun_util`](https://github.com/gwpl/rusty_things_organizer_4fun_util)


```
Usage: things [b|s]

Tool helps to organize where things are, using barcode reader.

Tool uses plaintext csv format, so it's easy to keep track with git or integrate with other scripts.

Assuming:

* alias things=/path/to/binary...
* thingsdb.csv sits in current working directory or is ok to be created

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


cabinet_container_000
container00
container10

container20
item20
<Ctrl-D>
...

# search (followed by things to search)
$ things s

Input:
item01
item11
container10
...

Output:
container00
container10
cabinet_container_000
...

```


In consideration for future if would turn out helpful:

```
# list things in container
$ things l container_code

# list containers
$ things lc

# tree view
$ things t

# move to container
$ things m container_code thing_code

# move to container multiple items
$ things m container_code
thing_code01
thing_code02
thing_code03

```
