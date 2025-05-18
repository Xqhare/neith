# Neith v2 Rework

## TODO

- [ ] Rewrite Readme.md
- [ ] Incorporate `Nabu`
    - [ ] General use of `XffValues`
- [ ] CRUD Log
- [ ] Decide if I want to change from a very sql like syntax
- [ ] Decide if I want to change from a very sql like database structure
- [ ] Keep ACID compliance
- [ ] Ability to upload a json file into the database
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
    "ACTION": CREATE | READ | UPDATE | DELETE | SAVE_FILE | LOAD_FILE,
    "CMD": "STRING",
    "USR_HASH": "HASH", // Don't currently know how I would implement this
    "DB_HASH": "HASH",
}
```

#### Keeping vs reworking the database structure and query language
One of the goals of this rework is to heed Uncle Bob's advice and not use `SQL` inside of programs.

On the other hand, I really like the style of querying I have already implemented.

If I keep it, the parser will still need to be rewritten from the ground up - easily done now that I have written my own file type.

##### Uncle Bob's Advice

> SQL was never intended to be used by computer programs. It was a console language for printing reports. Embedding it into programs was one of the gravest errors of our industry.
> — Uncle Bob Martin, 2025.04.30 on Twitter

It is clear to me, that I should at least entertain the Idea of a complete overhaul of NQL. And if I do so, I could also change the database structure as well.

One thing I am in favor of, is to split the query into several chained method calls like I did in `anansi`.

I am interpreting it from a lense of modern development, and the largest problem I always had with databases, was that I needed to keep the database structure in my mind, or open it for reference. Also I still need a `SQL` cheatsheet.
I think I could at least partially solve these problems.
Firstly by moving away form the one method call approach, to the chained method call approach.
Secondly by writing documentation for specific functions - The documentation will be displayed right inside the IDE.

##### Structure rework

###### `Document-oriented` vs `Key-Value`
The most interesting `nosql` database styles for me currently. \
Now, my understanding in the difference between the two is limited, but I believe the key difference to be that a `Document-oriented` database is a `Key-Value` database where not only the keys themselves are queried, but the contents of the Value can be evaluated as well inside a query.

I am currently favoring the move to a `Document-oriented` Database.

I do not like the name `Document`, and think I will replace it with `Object`.
`Object-oriented` Database is a term already in use, referring to storing something more like a programming object.

> `Neith` is a `nosql` database, utilising a `Document-oriented` database structure.

> `Neith` is a `nosql` database, utilising a `Object-focused` database structure.

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

If I encode the key-index like it is above, I forsee problems with the key-index size. For every level down, I need an entire byte.
Should I use it however, I could move from encoding buckets and collections as objects to encoding them as arrays.

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
