# NEITH: Neith Enhances Information Through Hierarchy
Neith is a small, lightweight and BLAZINGLY FAST database, written in and for rust.

It can be used as a normal run of the mill on disc database; saving and reading, to and from disc respectively. [More here!](#connecting)

It can also be used in `ram-mode` meaning that all data is held only in ram, if used this way Neith cannot save it's appstate. The data is however not encrypted and could be read from memory, so this is not a very secure database. [More here!](#connecting)

Neith has a `job_history` table that can be turned on, for saving some basic usage and duration logging. [More here!](#job-history)

> [!NOTE]
> It is not "Production-ready" and it will probably never be, I made this because I could, not because I should. I do consider it "stable" though, but take that with a grain of salt.
> While you can use it as a database, the developer experience is lacking as you can see further down in the example syntax.
> I am using Neith as a "stable" database for personal projects, but it remains with some un- or under-tested code. Other code did work when it was tested but may now be broken - It will be fixed if and when I find any bugs.
> I can only guarantee jank and maybe bugs, but that is a promise I can keep!

> [!IMPORTANT]
> If you really want to use Neith, please read this readme completely (especially this chapter), I tried my best explaining it.

The name Neith is derived from Neith, the ancient Egyptian goddess of war, creation, and weaving. She represents the intricate connections and patterns that form the cosmos and foundation of knowledge.

Neith is not made for large projects, or projects that need to do a lot of data intensive work. Neith is made for small projects, with the need for some database storage and simple logic. For large data-sets more ram is needed as Neith holds the entire database in memory from startup, leading to fast reads and writes (except the save to disc of course). Users are strongly discouraged from using complex API requests, this is mainly because it does not support multi-core - maybe at some point, no promises - so performance can be impacted by such requests.

My limited testing and experience has shown that Neith does quite well as long as the complexity and amount of data is managed, a simple table can hold 50k rows and while a slowdown is noticeable, it is still acceptable. For more complex tables the row-count is a fair bit lower at around 30k.
Splitting the data up into more tables inside Neith can help with performance too! A good rule of thumb is that the shorter the table the better the performance.
Please note that the more columns a table has, performance is impacted too. However, a table with a several thousand columns is out of the scope of this project anyway, so just keep it in mind - Neith is like any other database, more data complexity means more compute time.

Having said all this, Neith gives the perfect excuse for a bad performing program, as everyone knows that it's always the database's fault!

## Features

- small
- reasonably fast
- '.neithdb' is just '.json' making moving databases feasable
- lovingly handcrafted, no AI code!
- now with the v2 backend! And as 2 is twice as big as 1, it clearly is faster by the same factor!
- minimal dependencies (2 to be precise, chrono for timekeeping and json for json stuff)

## Design and philosophy of Neith

> [!IMPORTANT]
> Neith is un-opinionated and quite type-agnostic for a rust program. As such it will do whatever you tell it to do.
> It will only check for uniqueness of a value in a column if a column was marked as such.

Neith is designed to do what the user or program is telling it, whatever that is. Neith will execute anything passed to it, as long as it can decode it. There is no hand-holding, Neith will never assume or interpret what the user wants to do, it just does.

I tried to make it as un-opinionated as possible so that it will try to do whatever it is told to do; So beware of what you do!
For example, you can put whatever you want into any column, be it a number, string, boolean or list. This is by design, Neith will do what you tell it, and only inform you if it encountered an Error or succeeded.
These design principles are also the reason why Neith will not save to disc by itself.

To reiterate, Neith will not assume what it should do, it will wait for you to tell it what to do.

### ACID Compliance

ACID is a set of properties of database transactions intended to gurantee data validity despite errors and other mishaps.

#### Atomicity

> Atomicity gurantees that each transaction is treated as a single unit, which either succeeds of fails completely.

Neith treats each call of the `execute()` function as a transaction unit, and will return either a success message or error, depending on the state of the transaction.

#### Consistency

> Consitency ensures that a transaction can only bring the database from one consistent state to another, meaning that any data written to the database must be valid according to all defined rules.

Neith decodes and checks each query made by the user first, and only after confirming it to be a valid query Neith will execute it. This should prevent any illegal transaction leading to a ccorrupt database.

#### Isolation

> Isolation ensures that concurrent execution of transactions leaves the database in the same state that would have been obtained if the transactions were executed sequentially.

Neith cannot execute transactions concurrently.

#### Durability

> Durability guarantees that once a transaction has been committed, it will remain commited even in the case of a system failure.

Neith does not save the state to disc automatically, and if used in `ram-mode` it cannot save the state at all. So it is up to the user to ensure that a save to disc happens at appropriate points in their program.

Maybe I will implement a flag for automatic saving. This however is a [compute intensive operation](#saving-implementation), so it would probably default to `off` just like with the `job-history` table.

## Naming

> [!NOTE]
> Neith was an early ancient Egyptian deity said to be the first and the prime creator, who created the cosmos and all it contains, and that she governs how it functions.
> Her name was likely originally "nrt" or "She is the terrifying one".

Neith, the ancient Egyptian goddess, was a multifaceted deity revered for her roles in creation, wisdom, weaving, and war. She was one of the most enduring and influential goddesses throughout Egypt's long and storied history and was worshipped from the Pre-dynastic era (c. 6000-3150 BCE) through to the arrival of roman rule (30 BCE), some 4000 years.
Her primary association was with the city of Sais in the Nile river delta, where she was worshipped as the patron goddess. Often depicted as a woman wearing the red crown of lower Egypt and holding crossed arrows and bow, she symbolised power as well as war.
Neith's role as a creator goddess extended far beyond weaving, as she was believed to have woven the cosmos into existence on her loom. She was also associated with the primordial waters, further emphasising her creative power, as these waters were seen as the source of all life.
Her wisdom was also widely celebrated, and often consulted for guidance and counsel.

In addition Neith was also a fierce protector and warrior goddess. Her depictions often showed her with hunting implements, and her protective role extended into the afterlife where she was believed to guide and protect the deceased on their journey.

Her name resonates with the purpose of this database, which aims to weave together desperate pieces of information into a cohesive and meaningful tapestry of data. Just as Neith skilfully weaves threads into exquisite fabrics, this database seeks to seamlessly integrate and organise data to unveil hidden insights. Neith's association with order reflecting the database's ability to efficiently manage and organise data.

The name Neith embodies the essence of this database, symbolising it's ability to weave together, transform and illuminate the data, much like the goddess herself.

### Recursive Acronym
As with every project that takes itself seriously, the name is also a recursive acronym:

Neith

Enhances

Information

Through

Hierarchy

## Data-types

> [!IMPORTANT]
> It supports only basic data-types: floating point numbers, booleans, strings, as well as Lists.

Signed and unsigned integers are excluded for the sake of simplicity and ease of use.
If you really need to use them, Neith is probably not for you, or you could parse them, up to you really.

### Types:
Types are followed by their respective name in the API in parenthesis.
- Floats (float)
- Booleans (bool)
- Strings (string)
- Lists of any type (list) -> wrapped in (), e.g. (example, 1, true)

> [!TIP]
> Lists can contain up to five nested lists.

## API
Neith has a very simple API. It uses these functions, `connect()`, `execute()`, `set_marker()`, `set_job_history()` as well as `save()`.

`connect()` is only used once to create a connection to the database, for `ram-mode`, or a more detailed explanation check [here!](#connecting)

```
use neith::Neith;

let mut con = Neith::connect("DataBaseName");
```

Any interaction with Neith is done with the `execute()` function, this function uses Neithql or nql, a very simple and basic hand-rolled implementation of some sql syntax.
For examples on it's use [click here!](#data-interaction)

`save()` let's you can save the current state of the database to disc. If you are not running in `ram-mode` that is.

Example code:
```
use neith::Neith;

let mut con = Neith::connect("test");
let _ = con.save();
```
> [!TIP]
> Please note that `save()` needs the Neith object, which often involves cloning it (You know what that means better than I do!).

> [!IMPORTANT]
> Neith always returns something for each call. In most operations this is a simple success message containing a `true` boolean.
> The boolean wrapped by the `SuccessMessage` type does not matter; 
> It can also contain the data queried, or an Error encountered during execution.
> For this reason, it is recommended that you bind every query to a variable, marking it with `_` if you want to ignore the returned value.

> [!TIP]
> There is a simple [example database](#example-database) implementation. For more details please refer to this chapter, as it is mostly uncommented.

### Nql or Neith query language

Nql is a very simple sql and mysql inspired syntax for interacting with Neith.

#### Nql reference table

> [!TIP]
> The table is read left to right, take the position of the command chosen from the list, and use it as the index for any following lists.
> "!)" marks the end of a command and the "!" is NOT to be typed. Please note that following lists will have one entry less because of this.

> [!CAUTION]
> The table is always right, if any example code differs from it, it is wrong and needs to be changed.

| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 
| - | - | - | - | - | - | - | 
| execute( | new | table / column / data | 'tablename' | with / with / ('other_columnname' = 'new_data',+ 'different_column' = '(list,+ of,+ data,+ in,+ parenthesis)',+ ...)!) | ('columnname' 'unique', ...)!) / ('columnname' 'unique', ...)!)
| execute( | delete | table / column / data | with / with / in | 'tablename'!) / 'columnname' / 'tablename' | in / where | 'tablename'!) / ['columnname' = 'data',+ {and/not/or/xor} 'other_columnname' = 'other data',+ ...]!) |
| execute( | update | 'tablename' | where | ['columnname' = 'data',+ {and/not/or/xor} 'other_columnname' = '(other, data, as, list)',+ ...] | with | ('other_columnname' = 'new_data',+ 'different_column' = '(list,+ of,+ data,+ in,+ parenthesis)',+ ...)!) | 
| execute( | select | (columnname0, columnname1, ...)  OR * | from | 'tablename' | where | ['columnname' = 'data',+ {and/not/or/xor} 'other_columnname' = 'other data',+ ...]!) |
| execute( | get | min / max / len | in / in / of |  'columnname' / 'columnname' / 'tablename'!) | from / from | 'tablename'!) / 'tablename'!) |

##### Notes on using the reference table
The table is read left to right, please follow this example. After understanding how the table is used you will have learned all the nql syntax in existence!

By reading the table left to right in the first row, we start with `execute(` followed by `new`. The next field has 3 possibilities, `table`, `column` or `data`. Please note that the order of the elements does not change, so syntax need for `column` will always be second in the list, as long as any syntax is applicable.
With this in mind, we know that next we enter the `tablename`, and then choose the right next part in the correct place in the list. 
E.g. `data` was chosen, it is third in the list, so now `('other_columnname' = 'new_data',+ ...)!)` has to come next. 
The `!)` marks the end of the command, and the ! is NOT to be typed. It serves as a marker for ease of use during reference.
Each item of `name = data` has to be separated by `,+`. Neith splits the nql syntax in lists by this marker, so your data should not contain this.
The marker can be changed with the `con.set_marker("your_pattern_here")`. If used make sure to always execute and to do it as early as possible in your code.


Example code:
```
let mut con = Neith::connect("test");
let new_table = con.execute("new table testtable with (column1 true, column2 false, column3 false)");
let new_columns = con.execute("new column testtable with (column4 false, column5 false)");
let new_data_column1 = con.execute("new data testtable (column1 = 1,+ column2 = -2.04,+ column3 = true,+ column4 = text,+ column5 = (1.04,+ 2, false,+ more text))");
let new_data_column2 = con.execute("new data testtable (column1 = 2,+ column2 = -2.04,+ column3 = true,+ column4 = text,+ column5 = (1.04,+ 2,+ false,+ more text))");
let _ = con.save();
```

### Connecting

You can start Neith with one of three ways:

1. `connect("DBname")`
    - Most probably the way you want to start up Neith. It takes a `&str` as an argument for the Database name.
        - The name can be a valid path for control where the database is saved to, or read from. Please keep in mind that it has to be a valid path and filename with or without extension.
    - Setting of `job_history ` or `split_marker` with their own function-calls.
2. `connect_ram_mode(job_history: bool)`
    - The other way you could want to start up Neith. Doesn't need a name.
    - Setting of `job_history` in argument, `split_marker` with its own function-call.
3. `new(path: PathBuf, ram_mode: bool, job_history: bool)`
    - Needs a valid path to the database-location. Please keep in mind that it has to be a valid path and filename with or without extension.
    - Setting of `ram_mode` and `job_history` in arguments.

Example code:
```
// 1.
let mut connect = Neith::connect("test");
// 2.
let mut connect_ram_mode = Neith::connect_ram_mode(false);
// 3.
let mut new = Neith::new("MyPath", false, false);
```

#### Job History

Neih comes with a 'job-history' table that can be turned on during connection creation. This table saves the following:
As saving of this data can create unwanted ram and cpu overhead(Not much, however with the small scale of Neith, it could matter to you.), the feature is, by default, turned off.

- id (unique)
- command (the complete command typed in)
- time (the current date and time to the nanosecond)
- duration (how long the operation took in microseconds)

This table can be queried just like any other table. You can change the contents too, if you wish. Although that really isn't recommended.

> [!TIP]
> If you choose to use it, please use `set_job_history(true)` as the first thing after creating the connection.
> If you want to no longer use it, delete the line containing `set_job_history(true)` and also delete the table called `job_history` if you wish to do so.
> To query the `job_history`, just use the `execute()` function as you would with any other table.
> E.g. `execute("select * from job_history")`.

It can be turned on by: 

```
let con = Neith::connect("test");
let _ = con.set_job_history(true);
```

> [!CAUTION]
> Just treat it as read-only.
> Even though it is not. Trust me.
> The table is write-able, so you could do what you want with it. I would recommend against it though. So if something breaks it is your fault, I warned you.

#### Split marker

> [!IMPORTANT]
> By default it is `,+`. It can be changed to any string you desire using the `set_marker("your_pattern")` function.
> If changed, make sure to call as first thing at every startup.

> [!CAUTION]
> Some Neith-code is untested. This belongs to that category!

By default Neith splits some (please reference the table) lists up with a special split pattern, referred to as `marker`.
It is: `,+`.

This was done for better support of storing things like text-documents or code-snippets. If your data contains the default symbol Neith WILL mess up your data and write only up to the first occurrence of the split marker, whatever it is set to.

As this behaviour may not be preferable for every use-case I provided functionality to set it to any `String` you want.
The marker can be changed with the `con.set_marker("your_pattern_here")`. If used make sure to always execute and to do it as early as possible in your code.
For consistent results during parsing, please set as first thing during every start up.

Example code:
```
let con = Neith::connect("test");
let _ = con.set_marker(",");
```

### Data interaction

For data interaction of any kind the `execute()` function is used. It takes a `&str` as an argument and returns the appropriate data, a confirmation of success or error.
Example syntax is explained above in the [Nql reference table.](#nql-reference-table)

More detailed explanation in the following sub-chapters!

> [!NOTE]
> The `execute()` function will always return something, in most cases it is a simple `SuccessMessage` signalling that all went well.
> In other cases this will be the queried data or an error.

#### Writing data

Writing of data can be done in two ways, each dependent on your needs.

1. New data
2. Updating existing data

When to use each is easy, use new if you want to write a new entry into the table, and update if you want to update the data of an existing entry.

##### New table

To create a new table make use of the `execute()` function.

> [!NOTE]
> Each column needs a `unique_bool` boolean demarcating if the column contents will be unique (eg. the ID).
> Tables cannot be renamed, nor the name or unique boolean of their columns changed.

Neith does check if the table exists, and returns a success, however it will NOT write a table with the same name again.
Neith will treat two executions of the `"new table 'same_tablename'"` as one, however if the second execution would add more columns, they are ignored.
This is done because Neith works with the first table of any given name it has, but will save only the last table of that name.
That would be confusing behaviour so only one table per name is allowed.

Example code:
```
let con = Neith::connect();
let _new_table = con.execute("new table testtable with (column1 true, column2 false, column3 false)");
let _new_columns = con.execute("new column testtable with (column4 false, column5 false)");
```

The first line in the example above, establishes the database connection.
The second line creates a new table with the name `testtable` and the columns `column1`, `column2`, `column3`, with only `column1` containing `unique` values, e.g. an ID.
In the third line `testtable` is extended with `column4` and `column5`.

##### New data

Example code:
```
let mut con = Neith::connect("test");
let new_data_column1 = con.execute("new data testtable (column1 = 1,+ column2 = -2.04,+ column3 = true,+ column4 = text,+ column5 = (1.04,+ 2, false,+ more text))");
let new_data_column2 = con.execute("new data testtable (column1 = 2,+ column2 = -2.04,+ column3 = true,+ column4 = text,+ column5 = (1.04,+ 2,+ false,+ more text))");
```

The first line in the example above, establishes the database connection.
The second and third line write a new entry into `testtable` with the data supplied in parenthesis.

#### Updating data

Neith supports conditional statements for updating data. Supported are `and`, `not`, `xor`, and `or`.

Example code:
```
let con = Neith::connect("test.neithdb");
let update1 = con.execute("update testtable where [column2 = 1,+ and column4 = text] with (column3 = true)");
let update2 = con.execute("update testtable where [column2 = -2.04,+ or column2 = 1] with (column3 = false)");
let update3 = con.execute("update testtable where [column4 = text,+ not column2 = -2.04] with (column5 = (-1, 1.04, true, test text))");
```

The first line in the example above, establishes the database connection.
The second line updates every row in `testtable` where the conditions in square brackets are met, by setting `column3` to true.
The third line updates every row in `testtable` where the conditions in square brackets are met, by setting `column3` to false.
In the final line every row in `testtable` is updated by setting `column5` to the list (-1, 1.04, true, test text), where the conditions in square brackets are met.

#### Deleting data

You can delete rows, columns or entire tables.
There is nql syntax for each.

Example code:
```
let con = Neith::connect("test.neithdb");
let del_row = con.execute("delete data in testtable where [column1 = 4 and column4 = text]");
let del_column = con.execute("delete column with column5 and column4 in testtable");
let del_table = con.execute("delete table with testtable");
```

The first line in the example above, establishes the database connection.
The second line deletes an entire row, or entry, in `testtable` where the conditions in square brackets are met.
The third line deletes `column5` and `column4` in `testtable` with all their entries.
In the last line the table `testtable` is deleted.

#### Reading data

Neith supports conditional statements for querying data. Supported are `and`, `not`, `xor`, and `or`.

> [!NOTE]
> The * symbol is supported with the same usage as in sql, meaning 'all columns'.
> Select returns data ALWAYS in the order it was found in the table, e.g. if you search for 'column7, column1, column3' the results will be in the order 'column1, column3, column7'.

Example code:
```
let con = Neith::connect("test");
let select1 = con.execute("select * from testtable");
let select2 = con.execute("select (column1, column2, column3, column4) from testtable");
let select3 = con.execute("select (column1, column2) from testtable where [column1 = 1,+ and column2 = -2.04]");
```

The first line in the example above, establishes the database connection.
The second line selects all columns and entries from `testtable`.
The third line selects `column1`, `column2`, `column3`, and `column4` and all entries from `testtable`.
In the last line `column1`, `column2` are selected from `testtable` with the entry where the conditions in square brackets are met.

#### Convenience functions:

I have coded three "convenience" functions.

1. `get_min`
    - To get the minimum entry of any column.
2. `get_max`
    - To get the maximum entry of any column.
3. `get_len`
    - To get the length of any table. (Best used as id getter for table entries.)

Example code:
```
let con = Neith::connect("test.neithdb");
let get_min = con.execute("get min in column1 from testtable");
let get_max = con.execute("get max on column1 from testtable");
let get_len = con.execute("get len of testtable");
```

The first line in the example above, establishes the database connection.
The second line gets the minimum of all data in `column1` in `testtable`.
The third line gets the maximum of all data in `column1` in `testtable`.
In the last line the length of `testtable` is returned, meaning a count of the length, e.g a table with 0 entries would return 0, a table with 1 entry 1, ...

#### Saving data to disc

If Neith is set up using the `connect()` function, it will read any data found at the specified path, and do any operations on the data in ram. 
Ending the program without `.save()`-ing the connection, will not save the data from ram to disc, and will behave like a Neith instance in `ram-mode`, just without some benefits of the flag and slighlty more overhead.

> [!IMPORTANT]
> After a connection has been closed, it has to be reopened using the `connect()` function - this is resource intensive, so only save during run-time if necessary.

Example code:
```
let con = Neith::connect("test");
let _ = con.save();
```

This opens and immediately saves the state of Neith.

##### Saving implementation

Neith will save the database at the supplied path and the name during creation, with the extension `.neithdb`. This is just a `json` file, which is also the reason for subpar performance during saving and connecting of a medium to large database. This does also mean that a migration from Neith to almost any other database should be pretty easy.

## Example Database

This example generates a simple database with some employee information.
It reads, updates and deletes data.
It also saves this database to disc (before any deletion happens) so you can take a look at that too!
Because of this you can really see that if you don't pay attention to when you save, you can end up with a database that is no longer in sync with the data given to it.

```
// Constants for employee data generation:
let names = ["Joe Murica", "Mae Nada", "Hesus Xico", "Hermann Man", "Amelie Ance", "Mario Taly", "Maria Pain", "Peter Port", "Matt Ain", "Dirk Ands", "Dennis Mark", "Vitrali Pol"];
let jobs = ["Handyman", "Secretary", "Salesman", "Manager", "Cook", "Programmer", "Designer", "Artist", "CEO", "COO", "CFO", "CPO"];
let city = ["New York", "Montreal", "Mexico City", "Stuttgart", "Paris", "Siena", "Barcelona", "Lisbon", "Hull", "Amsterdam", "Copenhagen", "Gdansk"];
let gender = ["male", "female", "male", "male", "female", "male", "female", "male", "male", "male", "male", "male"];
let salary = [1500, 2500, 3000, 1250, 500, 6574, 5858, 9687, 6800, 5970, 5570, 4780];
let seniority = [1, 4, 8, 4, 5, 14, 19, 2, 24, 3, 16, 41];
let fav_colour = ["red", "green", "blue", "yellow", "red", "green", "blue", "yellow", "red", "green", "blue", "yellow"];

// Neith connection, set up and table creation
let mut con = Neith::connect("test");
let _change_marker = con.set_marker(",");
let _activate_history = con.set_job_history(true);
let _first_table = con.execute("new table testtable with (id true, full_name false, gender false, city false, job false)");
let _add_column = con.execute("new column testtable with (salary false, seniority false, colour false)");

// Data creation (12 times for a total of 12 employees!)
for employee in names.iter().enumerate() {
    let id = employee.0;
    let name = employee.1;
    let gender = gender[id];
    let city = city[id];
    let job = jobs[id];
    let salary = salary[id];
    let seniority = seniority[id];
    let colour = fav_colour[id];
    // remember, split pattern is set to ","!
    let cmd = format!("new data testtable (id = {id}, full_name = {name}, gender = {gender}, city = {city}, job = {job}, salary = {salary}, seniority = {seniority}, colour = {colour})");
    let _write_data = con.execute(&cmd);
}

// Data fetching and updating
let to_change_ids_read = con.execute("select (id) from testtable where [gender = male, and colour = red]");
// `get_result()` always returns a vector of all columns requested, in this case there is only one vector inside.
let change_ids = to_change_ids_read.unwrap().get_result().unwrap()[0].get_list().unwrap();
for id in change_ids {
    let decoded_id = id.get_float().unwrap();
    let _update0 = con.execute(format!("update testtable where [id = {decoded_id}] with (city = Bejing)").as_str());
    let changed_city = con.execute(format!("select (city) from testtable where [id = {decoded_id}]").as_str()).unwrap();
    // reading the changed state back out instantly
    let read_city = changed_city.get_result().unwrap()[0].get_list().unwrap();
    for city in read_city {
        let city_name = city.get_string().unwrap();
        assert_eq!(city_name, "Bejing".to_string());
    }
}
let all_employee_data = con.execute("select * from testtable").unwrap();
// Uncomment the print statements below to see how the data is returned
// println("{:?}", all_employee_data);
let read_data = all_employee_data.get_result().unwrap();
for column in read_data {
    let this_column = column.get_list().unwrap();
    // println("{:?}", this_column);
    assert_eq!(this_column.len(), 12);
}

// Reading the job table
let history = con.execute("select * from job_history").unwrap().get_result().unwrap();
let ids = history[0].get_list().unwrap();
let bindings = history[1].get_list().unwrap();
let dates = history[2].get_list().unwrap();
let durations = history[3].get_list().unwrap();
for id0 in ids {
    let id = id0.get_float().unwrap().to_string().parse::<usize>().unwrap();
    println!("(id = {id}, command = {:?}, time = {:?}, duration = {:?})", bindings[id], dates[id], durations[id]);
}

// Getters
let min_id = con.execute("get min in id from testtable").unwrap().get_result().unwrap()[0].get_float().unwrap();
assert_eq!(min_id, 0.0);
let max_id = con.execute("get max in id from testtable").unwrap().get_result().unwrap()[0].get_float().unwrap();
assert_eq!(max_id, 11.0);
let len_table = con.execute("get len of testtable").unwrap().get_result().unwrap()[0].get_float().unwrap();
assert_eq!(len_table, 12.0);

// Saving database
let _ = con.clone().save();

// Reseting file for next test
let del_data = con.execute("delete data in testtable where [gender = male, or city = Paris]").unwrap();
let del_column = con.execute("delete column with seniority in testtable").unwrap();
let del_table = con.execute("delete table with testtable").unwrap();

```


## Acknowledgments
Thanks to the open-source community for providing invaluable tools and libraries.
Used in this project:
- [json](https://crates.io/crates/json)
- [chrono](https://crates.io/crates/chrono)

