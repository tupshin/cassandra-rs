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
