# NEITH: Neith Enhances Information Through Hierarchy
Neith is a small, lightweight and BLAZINGLY FAST database.

The name Neith is derived from Neith, the ancient Egyptian goddess of war, creation, and weaving. She represents the intricate connections and patterns that form the cosmos and foundation of knowledge.

Neith is not made for large projects, or projects that need to do a lot of data intensive work. Neith is made for small projects, with the need for some database storage and simple logic. For large datasets more ram is needed as Neith holds the entire database in memory from startup, leading to fast reads and writes (except the save to disc of course). Users are strongly discuraged from using complex api requests, this is mainly because it does not support multicore - maybe at some point, no promises - so performance can be impacted by such requests.

## Design and philosophy of Neith

Neith is designed to do what the user or program is telling it, whatever that is. Neith will execute anything passed to it, as long as it can decode it. There is no handholding, Neith will never assume or interpret what the user wants to do, it just does.

I tried to make it as unopinionated as possible so that it will try to do whatever it is told to do; So beware of what you do!
For example, you can put whatever you want into any column, be it a number, string, boolean or list. Also the `unique` key entered during column creation is mostly for ease of use, Neith would happily put the same thing twice into a column marked as `unique`. This is by design, Neith will do what you tell it, and only inform you if it encountered an Error or succeded.
These design principles are the reason why Neith will not save to disc by itself for example. Neith will not assume what it should do, it will wait for you to tell it what to do.

## Naming

Neith, the ancient Egyptian goddess, was a multifaceted diety revered for her roles in creation, widom, weaving, and war. She was one of the most enduring and influential goddesses throughout Egypt's long and storied history and was worshipped from the Predynastic era (c. 6000-3150 BCE) through to the arrival of roman rule (30 BCE), some 4000 years.
Her primary association was with the city of Sais in the Nile river delta, where she was worshipped as the patron goddess. Often depicted as a woman wearing the red crown of lower egypt and holding crossed arrows and bow, she symbolized power as well as war.
Neith's role as a creator goddess extended far beyond weaving, as she was believed to have woven the cosmos into existence on her loom. She was also associated with the primordial waters, further emphasizing her creative power, as these waters were seen as the source of all life.
Her wisdom was also widely celebrated, and often consulted for guidance and counsel.

In additon Neith was also a fierce protector and warrior goddess. Her depictions often showed her with hunting implements, and her protective role extended into the afterlife where she was believed to guide and protect the deceased on their journey.

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
If you really need to use them, Neith is probably not for you.

### Types:
Types are followed by their respective name in the API in parenthesis.
- Floats (float)
- Booleans (bool)
- Strings (string)
- Lists of any type (list) -> wrapped in (), e.g. (example, 1, true)

###### On lists

Lists can contain up to five nested lists.

## API
Neith has a very simple API. It uses three functions, `connect()`, `execute()`, as well as `close()`.
The first is only used once to create a connection to the database, any interaction with it is done with the `execute()` function.
The `execute()` function uses Neithql or nql, a very simple and basic implementation of some sql syntax.
With the last function you can save the current state of the database to disc. If you are not running in ram-mode that is.

Note that Neith always returns something for each call. In most operations this is a simple success message containg a `true` boolean.
The boolean wrapped by the `SuccessMessage` type does not matter; 
It can also contain the data queried, or an Error encountered during execution.
For this reason, it is recommended that you bind every query to a variable, marking it with `_` if you want to ignore the returned value.

### Nql or Neith query language

Nql is a very simple sql and mysql inspired syntax for interacting with neith.

#### Nql reference table

| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 
| - | - | - | - | - | - | - | 
| execute( | new | table / column / data | 'tablename' | with / with / ('other_columnname' = 'new_data', ...)!) | ('columnname' 'unique', ...)!) / ('columnname' 'unique', ...)!) 
| execute( | delete | table / column / data | with / with / in | 'tablename'!) / 'columnname' / 'tablename' | in / where | 'tablename'!) / ['columnname' = 'data', {and/not/or/xor} 'other_columnname' = 'other data', ...]!) |
| execute( | update | 'tablename' | where | ['columnname' = 'data', {and/not/or/xor} 'other_columnname' = 'other data', ...] | with | ('other_columnname' = 'new_data', ...)!) | 
| execute( | select | (columnname0, columnname1, ...)  OR * | from | 'tablename' | where | ['columnname' = 'data', {and/not/or/xor} 'other_columnname' = 'other data', ...]!) |
| execute( | get | min / max / len | in / in / of |  'columnname' / 'columnname' / 'tablename'!) | from / from | 'tablename'!) / 'tablename'!) |

###### Notes on using the reference table
The table is read left to right, here the example for any `new` nql syntax:

By reading the table left to right in the first row, we start with 'execute(' followed by 'new'. The next field has 3 possibilites, 'table', 'column' or 'data'. Please note that the order of the elements does not change, so syntax need for 'coulumn' will always be second in the list, as long as any syntax is applicable.
With this in mind, we know that next we enter the 'tablename', and then choose the right next part in the correct place in the list. 
E.g. 'data' was choosen, it is third in the list, so now '('other_columnname' = 'new_data', ...)!)' has to come next. The '!)' marks the end of the command, and the ! is NOT to be typed.
It servers as a marker for ease of use during reference.

```
let mut con = Neith::connect("test.neithdb");
let new_table = con.execute("new table testtable with (column1 true, column2 false, column3 false)");
let new_columns = con.execute("new column testtable with (column4 false, column5 false)");
let new_data_column1 = con.execute("new data testtable (column1 = 1, column2 = -2.04, column3 = true, column4 = text, column5 = (1.04, 2, false, more text))");
let new_data_column2 = con.execute("new data testtable (column1 = 2, column2 = -2.04, column3 = true, column4 = text, column5 = (1.04, 2, false, more text))");
```

### Connecting
It is called with the `connection(path)` function, the returned type is the connection to the database.

### Data interaction
For data interaction of any kind the `execute()` function is used. It takes a `&str` as an argument and returns the appropriate data, a confirmation of success or error.
Example syntax is explained above.

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
let update2 = con.execute("update testtable where [column2 = -2.04 or column2 = 1] with (column3 = false)");
let update3 = con.execute("update testtable where [column4 = text not column2 = -2.04] with (column5 = (-1, 1.04, true, test text))");
```

Updates a single column entry of a table.

##### Rows

```
let con = Neith::connect("test.neithdb");
let new_data_column1 = con.execute("new data testtable (column1 = 1, column2 = -2.04, column3 = true, column4 = text, column5 = (1.04, 2, false, more text))");
let new_data_column2 = con.execute("new data testtable (column1 = 2, column2 = -2.04, column3 = true, column4 = text, column5 = (1.04, 2, false, more text))");
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
Select returns data ALWAYS in the oder it was found in the table, e.g. if you search for 'column7, column1, column3' the results will be in the order 'column1, column3, column7'.

#### Convinience functions:

```
let con = Neith::connect("test.neithdb");
let get_min = con.execute("get min in column1 from testtable");
let get_max = con.execute("get max on column1 from testtable");
let get_len = con.execute("get len of testtable");
```
#### Saving data to disc

If Neith is set up using the `connect()` function, it will read any data found at the specified path, and do any operations on the data in ram. Ending the programm without `.close()`-ing the connection, will not save the data from ram to disc, and will behave like a Neith instance in `ram-mode`, just without some benefits of the flag and some more overhead.
After a connection has been closed, it has to be reopened using the `connect()` function - this is ressource intensive, so only save during runtime if absolutely neccesarry.

```
let con = Neith::connect("test.neithdb");
con.close();
```
This opens and immmediatly closes (or saves the state of) neith.

WIP BELOW

### `job_history`-Table

Neih comes with a 'job-history' table that can be turned on during connection creation. This table saves the following:

- id (unique)
- command (the complete command typed in)
- time (the current date and time to the second)
- duration (how long the operation took in milliseconds)

This table can be queried just like any other table. You can change the contents too, if you wish. Although that really isn't recommended.
As this feature is experimental, it is off by default.

It can be turned on by: 

```
let con = Neith::connect("test.neithdb");
let job_his_on = con.set_job_history(true);
```

#### Notes on using the `job_history`-Table

If you choose to use it, please use `set_job_history(true)` as the first thing after creating the connection.
If you want to no longer use it, delete the line containing `set_job_history(true)` and also delete the table called `job_history` if you wish to do so.
To query the `job-history`, just use the `execute()` function as you would with any other table.

###### Notes on the `job_history`-Table

Just treat it as read-only.
Even though it is not. Trust me.
The table is writeable, so you could do what you want with it. I would recommend against it though.
If something breakes it is your fault, I warned you.
As saving of this data can create unwanted ram and cpu overhead(Not much, however with the small scale of Neith, it could matter to you.), the fearure is, by default, turned off.
