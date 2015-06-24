#![plugin(peg_syntax_ext)]

peg_file! cql("cql.rustpeg");

fn parse(stmt: &str) -> Result<i64, &str> {
    let result = match cql::cql_statement(stmt) {
        Ok(x) => Ok(0),
        _ => Err("meh")
    };
    result
}

fn verify(stmt: &str) {
    let result = parse(&stmt);
    assert!(result.is_ok());
}

#[test]
fn test_simple_select() {
    verify("select * from test");
}

#[test]
fn test_simple_select_fields_no_spacing() {
    verify("select field1,field2 from test");
}

#[test]
fn test_simple_select_with_spacing() {
    verify("select field1, field2 from test");
}

#[test]
#[should_panic]
fn test_invalid_selec() {
    verify("select from");
}


#[test]
fn test_fields() {
    assert!(cql::fields("name, age").is_ok());
}

#[test]
fn test_where_clause() {
    assert!(cql::predicate("term > ?").is_ok());
}

#[test]
fn test_where() {
    assert!(cql::where_clauses("where term > ?").is_ok());
}

#[test]
fn test_select_with_limit() {
    assert!(cql::cql_statement("select * from blah
                                LIMIT 1").is_ok());
    assert!(cql::cql_statement("select * from blah
                                WHERE a = ? and b = ?
                                LIMIT 1").is_ok());
}

#[test]
fn test_select_where() {
    assert!(cql::cql_statement("select * from tab where term > ?").is_ok());
    assert!(cql::cql_statement("select * from tab
                                where term > ?").is_ok());
    assert!(cql::cql_statement("select * from tab
                                where term > ? and x = ?").is_ok());
}

#[test]
fn test_select_order_by() {
    assert!(cql::cql_statement("select * from tab
                                where term > ?
                                order by bacon desc").is_ok());

    assert!(cql::cql_statement("select * from tab
                                where term > ?
                                order by bacon").is_ok());
}

#[test]
fn test_basic_insert() {
    assert!(cql::cql_statement("insert into blah (name, value)
                                values (?, ?)").is_ok());
}

#[test]
fn test_basic_delete() {
    assert!(cql::cql_statement("delete from blah where x = ?").is_ok());
}

#[test]
fn test_simple_update() {
    assert!(cql::cql_statement("update men set bal = ? where k = ?").is_ok());
}

#[test]
fn test_multiple_where_clauses() {
    assert!(cql::where_clauses("where k = ? and v = ?").is_ok());
}
#[test]
fn test_update_two_fields() {
    let tmp = cql::cql_statement("update men set bal = ?
                                  where k = ? and v = ?");
    assert!(tmp.is_ok());
}

#[test]
fn test_if_not_exists() {
    let q = "insert into users (name, age)
                values (?, ?)
                if not exists";
    assert!(cql::cql_statement(q).is_ok());

}

#[test]
fn test_ttl() {
    assert!(cql::using_clause("using ttl 60").is_ok());
    let q = "insert into users (name, age)
                values (?, ?)
                using ttl 60";
    assert!(cql::cql_statement(q).is_ok());

}


#[test]
fn test_timestamp() {
    assert!(cql::using_clause("using timestamp 60").is_ok());
    let q = "insert into users (name, age)
                values (?, ?)
                using timestamp 60";
    assert!(cql::cql_statement(q).is_ok());

}

#[test]
fn test_if_clause() {
    let tmp = cql::cql_statement("update men set bal = ?
                                  where k = ? if bal = ?");
    assert!(tmp.is_ok());

}

#[test]
fn test_update_using() {
    let tmp = cql::cql_statement("update men
                                    using ttl 60
                                    set bal = ?
                                  where k = ? ");
    assert!(tmp.is_ok());

}

// counters
#[test]
fn test_counters() {
    cql::counter_op("blah = blah + 1");
    cql::counter_op("blah = blah - 1");
    cql::counter_op("blah = blah - ?");
    let q = "update whatever
             set k = k + 1
             where bacon = ?";
    assert!(cql::cql_statement(q).is_ok());
}

// lists

// sets

// maps

// in()


// delete using timestamp

// delete using ttl
