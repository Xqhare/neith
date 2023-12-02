# NEITH: Neith Enhances Information Through Hierarchy
Neith is a small, lightweight and BLAZINGLY FAST database.

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

### Connecting
It is called with the `connection(path)` function, the returned type is the connection to the database.

### Data interaction
For data interaction of any kind the `execute()` function is used. It takes a `&str` as an argument and returns the appropriate data, a confirmation of success or error.
Example syntax is explained further down, this is a reference table.

| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 
| - | - | - | - | - | - | - | - 
| execute( | new | table / row | 'tablename' | with | 'rowname', optional_uniqe | and | 'other_rowname', optional_unique) 
| execute( | delete | table / row | with | 'tablename' / 'rowname' | in | 'tablename') |
| execute( | update | 'tablename' | where | 'rowname' = 'data' and... | with | 'other_rowname' = 'new_data' and...) | 
| execute( | select | 'rowname' and... / * | from | 'tablename' | where | 'rowname' = 'data' and...) |
| execute( | get | min / max / len | in / of |  'rowname' / 'tablename' | from | 'tablename') |
#### Writing data

##### Tables

This creates a table with the name `table_name` and the columns `column_name0` and `column_name1`. 
Each column needs a `unique_bool` boolean demarcating if the column contents will be unique (eg. the ID).
###### Notes on tables
Tables cannot be renamed, nor the name or unique boolean of their columns changed.

##### Rows
Updates a single column entry of a table.


#### Deleting data

##### Tables

##### Rows
Deletes an entire row.
`delete("{table_name} where {collum_name} = {value}")`

#### Reading data
Selects entry in specified column. * is valid for all columns.
Reading more than one column is not supported.
`select("{column_name} from {table_name} where {other_column_name} = {value}")`

#### Convinience functions:
Returns the maximum value of a given row.
`max(collum_name)`
Returns the minimum value of a given row.
`min(collum_name)`
Returns the length of a table.
`len(table_name)`
