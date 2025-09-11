use chumsky::prelude::*;

#[inline(never)]
pub fn parse(input: &str) -> Result<String, ()> {
    parser().parse(input).into_result().map_err(|_| ())
}

#[inline]
fn parser<'src>() -> impl Parser<'src, &'src str, String, extra::Default> {
    let escape_character = just('\\').ignore_then(choice((
        just('\\').to('\\'),  // 反斜杠
        just('t').to('\t'),   // 制表符
        just('n').to('\n'),   // 换行符
        just('r').to('\r'),   // 回车符
        just('0').to('\0'),   // 空字符
        just('"').to('"'),    // 双引号
        just('\'').to('\''),  // 单引号
        just('b').to('\x08'), // 退格符
        just('f').to('\x0C'), // 换页符
        just('v').to('\x0B'), // 垂直制表符
        just('a').to('\x07'), // 响铃符
    )));

    none_of('\\').or(escape_character).repeated().collect()
}

#[test]
fn test_parse() {
    let s = parse(r#"hello\\\n\tniho\r\"\'\0\bxl\a\n123\b2"#).unwrap();
    println!("{}", s);
}
