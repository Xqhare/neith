use crate::Neith;

#[test]
fn a_read_neithdb_file() {
    let test = Neith::connect("test.neithdb");
    let test_json = Neith::connect("test.json");
    assert_eq!(test, test_json);
}
#[test]
fn b_test_execute_new() {
    let con = Neith::connect("test.neithdb");
    let new_table = con.execute("new table testtable with (column1 true, column2 false, column3 false)");
    let new_column = con.execute("new column testtable with column4 and column5, unique");
    let new_data_column1 = con.execute("new data testtable (column1, column2, column3, column4, column5) (1, -2.04, true, text, (1.04, 2, false, more text))");
    let new_data_column2 = con.execute("new data testtable (column1, column2, column3, column4, column5) (2, -2.04, true, text, (1.04, 2, false, more text))");
    let new_data_column3 = con.execute("new data testtable (column1 = 3, column2 = 1, column4 = text)");
    let new_data_column4 = con.execute("new data testtable (column1 = 4, column2 = 1, column4 = text)");
    assert!(new_column.unwrap() == new_table.unwrap() && new_data_column1.unwrap() == new_data_column2.unwrap() && new_data_column3.unwrap() == new_data_column4.unwrap());
}
#[test]
fn c_test_execute_update() {
    let con = Neith::connect("test.neithdb");
    let update1 = con.execute("update testtable where [column2 = 1 and column4 = text] with (column3 = true)");
    let update2 = con.execute("update testtable where [column2 = -2.04 or column2 = 1] with (column3 = false)").unwrap();
    let update3 = con.execute("update testtable where [column4 = text not column2 = -2.04] with (column5 = (-1, 1.04, true, test text))");
    assert!(update1.unwrap() == update2 && update2 == update3.unwrap());
}
#[test]
fn d_test_execute_select() {
    let con = Neith::connect("test.neithdb");
    let select1 = con.execute("select * from testtable");
    let select2 = con.execute("select (column1, column2, column3, column4) from testtable");
    assert!(select1.unwrap() == select2.unwrap());
}
#[test]
fn e_test_execute_get() {
    let con = Neith::connect("test.neithdb");
    let get_min = con.execute("get min in column1 from testtable");
    let get_max = con.execute("get max on column1 from testtable").unwrap();
    let get_len = con.execute("get len of testtable").unwrap();
    // len and max should? be the same length later too!
    assert!(get_len == get_max && get_max == get_min.unwrap());
}
#[test]
fn f_test_execute_delete() {
    let con = Neith::connect("test.neithdb");
    let _del_row = con.execute("delete data in testtable where [column1 = 4 and column4 = text]");
    let del_column = con.execute("delete column with column5 and column4 in testtable");
    let del_table = con.execute("delete table with testtable");
    assert!(del_column.unwrap() == del_table.unwrap());
}
