use crate::Neith;

#[test]
fn test_main_connection() {
    // CONST:
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
    let first_table = con.execute("new table testtable with (id true, full_name false, gender false, city false, job false)");
    let add_column = con.execute("new column testtable with (salary false, seniority false, colour false)");
    assert_eq!(first_table.unwrap(), add_column.unwrap());
    // Data creation (12 times!)
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
        let _wirte_data = con.execute(&cmd);
    }
    // let _ = con.clone().save();
    // Data fetching and updating
    let to_change_ids_read = con.execute("select (id) from testtable where [gender = male, and colour = red]");
    let change_ids = to_change_ids_read.unwrap().get_result().unwrap()[0].get_list().unwrap();
    for id in change_ids {
        let decoded_id = id.get_float().unwrap();
        let _update0 = con.execute(format!("update testtable where [id = {decoded_id}] with (city = Bejing)").as_str());
        let changed_city = con.execute(format!("select (city) from testtable where [id = {decoded_id}]").as_str()).unwrap();
        let read_city = changed_city.get_result().unwrap()[0].get_list().unwrap();
        for city in read_city {
            let city_name = city.get_string().unwrap();
            assert_eq!(city_name, "Bejing".to_string());
        }
    }
    let all_employee_data = con.execute("select * from testtable").unwrap();
    let read_data = all_employee_data.get_result().unwrap();
    for column in read_data {
        let this_column = column.get_list().unwrap();
        assert_eq!(this_column.len(), 12);
    }
    // Getters
    let min_id = con.execute("get min in id from testtable").unwrap().get_result().unwrap()[0].get_float().unwrap();
    assert_eq!(min_id, 0.0);
    let max_id = con.execute("get max in id from testtable").unwrap().get_result().unwrap()[0].get_float().unwrap();
    assert_eq!(max_id, 11.0);
    let len_table = con.execute("get len of testtable").unwrap().get_result().unwrap()[0].get_float().unwrap();
    assert_eq!(len_table, 12.0);
    // Reseting file for next test
    let del_data = con.execute("delete data in testtable where [gender = male, or city = Paris]").unwrap();
    let del_column = con.execute("delete column with seniority in testtable").unwrap();
    let del_table = con.execute("delete table with testtable").unwrap();
    assert_eq!(del_data, del_column);
    assert_eq!(del_column, del_table);
}
