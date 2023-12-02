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
    let new_table = con.execute("new table testtable with row1, unique and row2 and row3");
    let new_row = con.execute("new row testtable with row4 and row5, unique");
    let new_data_row1 = con.execute("new data testtable (row1, row2, row3, row4, row5) (1, -2.04, true, text, (1.04, 2, false, more text))");
    let new_data_row2 = con.execute("new data testtable (row1, row2, row3, row4, row5) (2, -2.04, true, text, (1.04, 2, false, more text))");
    let new_data_row3 = con.execute("new data testtable (row1 = 3, row2 = 1, row4 = text)");
    let new_data_row4 = con.execute("new data testtable (row1 = 4, row2 = 1, row4 = text)");
    assert!(new_row.unwrap() == new_table.unwrap() && new_data_row1.unwrap() == new_data_row2.unwrap() && new_data_row3.unwrap() == new_data_row4.unwrap());
}
#[test]
fn c_test_execute_update() {
    let con = Neith::connect("test.neithdb");
    let update1 = con.execute("update testtable where [row2 = 1 and row4 = text] with (row3 = true)");
    let update2 = con.execute("update testtable where [row2 = -2.04 or row2 = 1] with (row3 = false)").unwrap();
    let update3 = con.execute("update testtable where [row4 = text not tow2 = -2.04] with (row5 = (-1, 1.04, true, test text))");
    assert!(update1.unwrap() == update2 && update2 == update3.unwrap());
}
#[test]
fn d_test_execute_select() {
    let con = Neith::connect("test.neithdb");
    let select1 = con.execute("select * from testtable");
    let select2 = con.execute("select (row1, row2, row3, row4) from testtable");
    assert!(select1.unwrap() == select2.unwrap());
}
#[test]
fn e_test_execute_get() {
    let con = Neith::connect("test.neithdb");
    let get_min = con.execute("get min in row1 from testtable");
    let get_max = con.execute("get max on row1 from testtable").unwrap();
    let get_len = con.execute("get len of testtable").unwrap();
    // len and max should? be the same length later too!
    assert!(get_len == get_max && get_max == get_min.unwrap());
}
#[test]
fn f_test_execute_delete() {
    let con = Neith::connect("test.neithdb");
    let del_row = con.execute("delete row with row5 and row4 in testtable");
    let del_table = con.execute("delete table with testtable");
    assert!(del_row.unwrap() == del_table.unwrap());
}
