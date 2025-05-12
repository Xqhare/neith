# Neith v2 Rework

## TODO

- [ ] Rewrite Readme.md
- [ ] Incorporate `Nabu`
    - [ ] General use of `XffValues`
- [ ] CRUD Log
- [ ] Decide if I want to change from a very sql like syntax
- [ ] Decide if I want to change from a very sql like database structure
- [ ] Keep ACID compliance

### Notes

#### Keeping vs reworking the database structure and query language
One of the goals of this rework is to heed Uncle Bob's advice and not use sql inside of programs.

On the other hand, I really like the style of querying I have already implemented.

If I keep it, the parser will need to be rewritten from the ground up - easily done now that I have written my own file type.

#### CRUD Log
A CRUD Log, as proposed in xff spec 3.
As opposed to xff spec v3, the log is stored in its own file / directory.

```
"crud_log": {
    "UNIX_TIMESTAMP": "TIMESTAMP",
    "ACTION": CREATE | READ | UPDATE | DELETE | SAVE_FILE | LOAD_FILE,
    "CMD": "STRING",
    "USR_HASH": "HASH",
}
```
