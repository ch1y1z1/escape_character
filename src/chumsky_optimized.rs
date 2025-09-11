use chumsky::{container::Container, prelude::*};

#[inline(never)]
pub fn parse(input: &str) -> Result<String, ()> {
    parser().parse(input).into_result().map_err(|_| ())
}

struct CollectString(String);

impl Default for CollectString {
    fn default() -> Self {
        CollectString(String::new())
    }
}

impl Container<&str> for CollectString {
    fn push(&mut self, item: &str) {
        self.0.push_str(item);
    }
}

#[inline]
fn parser<'src>() -> impl Parser<'src, &'src str, String, extra::Default> {
    let escape_character = just('\\')
        .ignore_then(choice((
            just('\\').to("\\"),  // 反斜杠
            just('t').to("\t"),   // 制表符
            just('n').to("\n"),   // 换行符
            just('r').to("\r"),   // 回车符
            just('0').to("\0"),   // 空字符
            just('"').to("\""),   // 双引号
            just('\'').to("\'"),  // 单引号
            just('b').to("\x08"), // 退格符
            just('f').to("\x0C"), // 换页符
            just('v').to("\x0B"), // 垂直制表符
            just('a').to("\x07"), // 响铃符
        )))
        .map(|ch| ch);

    let raw = none_of('\\').repeated().at_least(1).to_slice();

    raw.or(escape_character)
        .repeated()
        .collect()
        .map(|CollectString(s)| s)
}

#[test]
fn test_parse() {
    let s = parse(r#"hello\\\n\tniho\r\"\'\0\bxl\a\n123\b2"#).unwrap();
    println!("{}", s);
}
