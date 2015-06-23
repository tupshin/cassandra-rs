#![plugin(peg_syntax_ext)]

peg_file! cql("cql.rustpeg");

fn parse(stmt: &str) -> Result<i64, &str> {
    Result::Ok(0)
}

fn verify(stmt: &str) {
    let result = parse(&stmt);
    assert!(result.is_ok());
}

#[test]
fn test_simple_select() {
    assert!(false);
}
