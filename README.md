# NEITH: Neith Enhances Information Through Hierarchy
Neith is a small, lightweight and BLAZINGLY FAST database.

Neith is not made for large projects, or projects that need to do a lot of data intensive work. Neith is made for small projects, with the need for some database storage and simple logic. For large datasets more ram is needed as Neith holds the entire database in memory from startup, leading to fast reads and writes (except the save to disc of course). Users are strongly discuraged from using complex api requests, this is mainly because it does not support multicore - maybe at some point, no promises - so performance can be impacted by such requests.

I tried to make it as unopinionated as possible so it will try to do whatever it is told to do; So beware of what you tell it to do!

## Naming

The name Neith is derived from Neith, the ancient Egyptian goddess of war, creation, and weaving. She represents the intricate connections and patterns that form the cosmos and foundation of knowledge. 

Her name resonates with the purpose of this database, which aims to weave together desperate peices of information into a cohesive and meaningful tapestry of data. Just as Neith skillfully weaves threads into exquisite fabrics, this database seeks to seamlessly integrate and organize data to unveil hidden insights. Neith's association with order reflecting the databese's ability to efficiently manage and organize data.

The name Neith embodies the essence of this database, symbolizing it's ability to weave together, transform and illuminate the data, much like the goddess herself.

### Recursive Acronym
The name is also a recursive arconym:

Neith
Enhances
Information
Through
Hierarchy

## Datatypes
It supports only basic datatypes, floating point numbers, booleans, strings, as well as Lists.
Signed and unsigned intergers are excluded for the sake of simplicity, ease of use and a smaller footprint.
If you really need to use them, Neith is not for you.

### Types:
Types are followed by their respective name in the API in parenthesis.
- Floats (float)
- Booleans (bool)
- Strings (string)
- Lists of any type (list)

## API
Neith has a very simple API. It uses two functions, `connect()`, as well as `execute()`.
The first is only used once to create a connection to the database, any interaction with it is done with the `execute()` function.
The `execute()` function uses Neithql or nql, a very simple and basic implementation of some sql syntax.

### Connecting
It is called with the `connection(path)` function, the returned type is the connection to the database.

### Data interaction
For data interaction of any kind the `execute()` function is used. It takes a `&str` as an argument and returns the appropriate data, a confirmation of success or error.
Example syntax is explained further down, this is a reference table.

| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 
| - | - | - | - | - | - | - | 
| execute( | new | table / column / data | 'tablename' | with / with / (columnname0, columnname1, ...) OR ('other_columnname' = 'new_data', ...) | ('rowname' 'unique', ...) / ('rowname' 'unique', ...) / values (val0, val1, ...)) 
| execute( | delete | table / column / row | with / with / in | 'tablename' / 'columnname' / 'tablename' | in / where | 'tablename' / ['columnname' = 'data', {and/not/or} 'other_columnname' = 'other data', ...]) |
| execute( | update | 'tablename' | where | ['columnname' = 'data', {and/not/or} 'other_columnname' = 'other data', ...] | with | ('other_columnname' = 'new_data', ...)) | 
| execute( | select | (columnname0, columnname1, ...)  / * | from | 'tablename' | where | ['columnname' = 'data', {and/not/or} 'other_columnname' = 'other data', ...]) |
| execute( | get | min / max / len | in / in / of |  'columnname' / 'columnname' / 'tablename' | from | 'tablename') |

#### Writing data

##### Tables
```
let con = Neith::connect("test.neithdb");
let new_table = con.execute("new table testtable with (column1 true, column2 false, column3 false)");
```
This creates a table with the name `testtable` and the columns `column1`, `column2` and `column3`. 

Each column needs a `unique_bool` boolean demarcating if the column contents will be unique (eg. the ID).

###### Notes on tables
Tables cannot be renamed, nor the name or unique boolean of their columns changed.

##### Columns
```
let con = Neith::connect("test.neithdb");
let new_column = con.execute("new column testtable with column4 and column5, unique");
```
This extends the created `testable` with `column4` and `column5`.

```
let con = Neith::connect("test.neithdb");
let update1 = con.execute("update testtable where [column2 = 1 and column4 = text] with (column3 = true)");
let update2 = con.execute("update testtable where [column2 = -2.04 or column2 = 1] with (column3 = false)").unwrap();
let update3 = con.execute("update testtable where [column4 = text not column2 = -2.04] with (column5 = (-1, 1.04, true, test text))");
```

Updates a single column entry of a table.

##### Rows

```
let con = Neith::connect("test.neithdb");
let new_data_column1 = con.execute("new data testtable (column1, column2, column3, column4, column5) (1, -2.04, true, text, (1.04, 2, false, more text))");
let new_data_column2 = con.execute("new data testtable (column1, column2, column3, column4, column5) (2, -2.04, true, text, (1.04, 2, false, more text))");
let new_data_column3 = con.execute("new data testtable (column1 = 3, column2 = 1, column4 = text)");
let new_data_column4 = con.execute("new data testtable (column1 = 4, column2 = 1, column4 = text)");
```

#### Deleting data

##### Tables

```
let con = Neith::connect("test.neithdb");
let _del_row = con.execute("delete row in testtable where [column1 = 4 and column4 = text]");
let del_column = con.execute("delete column with column5 and column4 in testtable");
let del_table = con.execute("delete table with testtable");
```

##### Rows
Deletes an entire row.

#### Reading data

```
let con = Neith::connect("test.neithdb");
let select1 = con.execute("select * from testtable");
let select2 = con.execute("select (column1, column2, column3, column4) from testtable");
```

Selects entry in specified column. * is valid for all columns.

#### Convinience functions:

```
let con = Neith::connect("test.neithdb");
let get_min = con.execute("get min in column1 from testtable");
let get_max = con.execute("get max on column1 from testtable");
let get_len = con.execute("get len of testtable");
```

