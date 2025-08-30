# Neith v2 Rework

## TODO

- [ ] Rewrite Readme.md
- [ ] Incorporate `Nabu`
    - [ ] General use of `XffValues`
    - [ ] Decide whether to wait for version 3 of `Nabu`
- [ ] CRUD Log
- [ ] Decide if I want to change from a very sql like syntax
- [ ] Decide if I want to change from a very sql like database structure
- [ ] Keep ACID compliance
- [ ] Ability to upload and download a json file into the database
    - [ ] Ability to construct a bucket, collection or object from a json file
    - [ ] Ability to download a json file from a bucket, collection or object

### Notes

#### CRUD Log
A CRUD Log, as proposed in xff spec 3.
As opposed to xff spec v3, the log is stored in its own file / directory.

Use a feature flag?

```
"crud_log": {
    "UNIX_TIMESTAMP": "TIMESTAMP",
    "ACTION": CREATE | READ | UPDATE | DELETE | CREATE_FILE | READ_FILE | UPDATE_FILE | DELETE_FILE,
    "CMD": "STRING",
    "USR_HASH": "HASH", // Don't currently know how I would implement this
    "DB_HASH": "HASH",
}
```

##### `ACTION`
The CRUD operations on data are a no-brainer.
File CRUD operations are a bit more complicated:

I still haven't decided on the file structure of the database.

- There are probably two main options:
    1. One file per bucket
    2. One file per collection
- I would want the crud log as high up the tree as possible.

#### File structure
Pos1 one file per bucket:
```
db_root/
    crud_log/
        UNIX_TIMESTAMP_C.xff
        // C = Create
        // R = Read
        // U = Update
        // D = Delete
        // CF = Create File
        // RF = Read File
        // UF = Update File 
        // DF = Delete File
    buckets/
        bucket_frequency.xff
        bucketname.xff
        anotherbucketname.xff
```

Pos2 one file per collection:
```
db_root/
    crud_log/
        UNIX_TIMESTAMP_C.xff
    buckets/
        bucket_frequency.xff
        bucketname/
            collection_frequency.xff
            collectionname.xff
            anothercollectionname.xff
```

There are more things to consider:

- key-frequency store?
- CRUD-Log

#### Keeping vs reworking the database structure and query language
One of the goals of this rework is to heed Uncle Bob's advice and not use `SQL` inside of programs.

On the other hand, I really like the style of querying I have already implemented.

If I keep it, the parser will still need to be rewritten from the ground up - easily done now that I have written my own file type.

##### Uncle Bob's Advice

> SQL was never intended to be used by computer programs. It was a console language for printing reports. Embedding it into programs was one of the gravest errors of our industry.
> — Uncle Bob Martin, 2025.04.30 on Twitter

It is clear to me, that I should at least entertain the Idea of a complete overhaul of NQL. And if I do so, I could also change the database structure as well.

One thing I am in favor of, is to split the query into several chained method calls like I did in `anansi`.

I am interpreting it from a lense of modern development, and the largest problem I always had with databases, was that I needed to keep the database structure in my mind, or open it for reference. Also, I still need a `SQL` cheatsheet.
I think I could at least partially solve these problems.
Firstly by moving away form the one method call approach, to the chained method call approach.
Secondly by writing documentation for specific functions - The documentation will be displayed right inside the IDE.

Anything displaying to the developer as they write code should be the highest priority for the documentation, as this is where it is actually the most useful.

##### Structure rework

I have somehow gotten the Idea to hold as much data as I can on disc - if `neith` is not in `ram-mode`.
This would need to be paired with some kind of caching - and I was afraid this rework would be easy!

Every key-value would need a frequency store, so I can keep track of the hottest keys and cache them.

I could also create a key-frequency store for the whole database - that could even be saved to disc!
That could be used to construct the cache with some frequency or size limit to read more data to the cache.

If I allow the user to set a custom cache size, that would maybe be the easiest for me? If they pass `0` I could basically run in `ram-mode`.
Should I load a file that is larger than the cache size allows, I could skip without opening the file?

The system would also need to cope with data on disk, and I would like that to be a nice abstraction layer to keep `ram-mode` as kind of the standard way of running `neith` and `persistancy` is a feature.

The cache would basically mirror the directory structure of the database and search through only the cached data, returning `None` if the data is not cached. Then I can start the expensive disk checking.
I could also just limit it to the file level, so any data held by the bucket or collection, whatever option I end up going with, would be cached.

It would probably be prudent to calculate the cache whilst running - instead of doing this automagically just expose a function to do it.
This turns the entire thing into a startup only thing.

The size of the cache should be checked against any file to be read in, if it (and the current cache) exceeds the size it simply ends the startup caching.

Limiting the cache size should only be possible outside `ram-mode`.

No dynamic cache size, only statically allocated.

###### `Document-oriented` vs `Key-Value`
The most interesting `nosql` database styles for me currently. \
Now, my understanding in the difference between the two is limited, but I believe the key difference to be that a `Document-oriented` database is a `Key-Value` database where not only the keys themselves are queried, but the contents of the Value can be evaluated as well inside a query.

I am currently favoring the move to a `Document-oriented` Database.

I do not like the name `Document`, and think I will replace it with `Object`.
`Object-oriented` Database is a term already in use, referring to storing something more like a programming object.

> `Neith` is a `nosql` database, utilising a `Document-oriented` database structure.

> `Neith` is a `nosql` `Key-Value` database, utilising a `Object-focused` database structure.
> It is mainly designed to be a database for a single program or project.

This is mainly because of all my work on `nabu`.

The structure would look like this:

```
{
    buckets: {
        "bucket1": {
            collections: {
                "collection1": {
                    "object1": {
                        "key": XffValue,
                    },
                    "key": XffValue,
                }
            }
        }
    }
}
```

There are some open questions though:

1. What should be the file structure for the database?
    - Monolithic?
    - One file per bucket?
    - One file per collection?
        - One file per collection + One dir per bucket?
2. Add compression?
    - A lot of overhead, especially if using my hand-rolled compression; No other would do though.

As it stands right now, I seem to be working towards a unique flavor or spin on a `Document-oriented` Database.

Create a Key index for faster querying?
This would need to be created on every startup?
Would also force the use of a HashSet instead of a Map, but this could enable key-index persistence (But nabu makes that a lot harder).
Persistence could also be achieved by creating a file that stores the key-index as an obj.

This opens the question of key uniqueness, which I would like to keep as it is - Keys only need to be unique within a bucket, collection or object.
This could still be achieved by using a Key index, instead of index it would store all?

```
{
    buckets: {
        "key": {
            collections: {
                "key": {
                    "key": {
                        "key": XffValue,
                    },
                }
            }
        }
        "bucket": {
            collections: {
                "collection": {
                    "object": {
                        "key": XffValue,                        
                    },
                }
            }
        }
    }
}

key-index: {
    "key": vec![
        vec![0], // first bucket
        vec![0, 0], // first bucket, first collection
        vec![0, 0, 0], // first bucket, first collection, first object
        vec![0, 0, 0, 0], // first bucket, first collection, first object, first key
        vec![1, 0, 0, 0], // second bucket, first collection, first object, first key
    ]
}
```

If I encode the key-index like it is above, I for-see problems with the key-index size. For every level down, I need an entire byte.
Should I use it however, I could move from encoding buckets and collections as objects to encoding them as arrays.

I could rethink the key-index into a simpler affair:
Move away from the position in an array, and move to something more akin to the `nabu` v3 proposed string deduplication.
This paired with the v3 proposed frequency analysis could be awesome, but the frequency analysis itself could prove to be a problem.
Changing keys throughout the database as frequency changes looks like a pain to implement.

In the end, the frequency analysis would need to be dropped. Then this option would be a simple key-integer index.
The size of the integer dictates the possible maximum amount of keys the database can hold.
`u32` -> around 4.2B keys
`u64` -> around 18.4 quintillion keys

Now, a `u32` feels a bit low, but a `u64` is ludicrous.
But if somebody actually hits that, they have got no-one to blame but themselves for using `Neith`.
Also, if `neith` somehow gets a larger user-base, someone surely will make the upgrade at some point - can't be that hard.

This would also mean however that all keys of the entire database need to be unique, so I can map them to a unique integer.
Absolute non-starter, how would I save several pictures with a `extension` key?

I could solve this by moving the key-index into every bucket, collection and object.
This is somewhat stupid, as I am now using two HashMap lookups per level.

NACK - Key-Index

##### NQL rework

```
// high level structure
let db = Neith::connect("test");
let bucket = db.bucket("test-bucket");
let collection = bucket.collection("test-collection");
let object = collection.object("test-object");

// low level structure
object = {
    "key": XffValue,
}

// querying

// returns all Values with the key "key"
let res1 = db.get("key");
let res2 = bucket.get("key");
let res3 = collection.get("key");
let res4 = object.get("key");

```

I want "$gt", "$gte", "$lt", "$lte", "$eq", "$neq", "$and", "$or", "$not" for stuff like:
```
let res = db.find({"name": "Test", "extension": "txt", "data": {"$gt": 1000}});

// will probably look more like this:
let res = db.find([("name", "Test"), ("extension", "txt"), ("data", "$gt: 1000")]);
// This would return all objects with a name of "Test", an extension of "txt" and a data value greater than 1000 
```
