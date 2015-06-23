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
}
