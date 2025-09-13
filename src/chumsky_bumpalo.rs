use crate::io::Output;
use bumpalo::{Bump, collections::String as BString};
use chumsky::{container::Container, prelude::*};
use ouroboros::self_referencing;

#[inline(never)]
pub fn parse(input: &str) -> Result<impl Output, ()> {
    parser().parse(input).into_result().map_err(|_| ())
}

#[self_referencing]
pub struct CollectString {
    b: Bump,
    #[borrows(b)]
    #[covariant]
    s: BString<'this>,
}

impl<'b> Default for CollectString {
    fn default() -> Self {
        CollectStringBuilder {
            b: Bump::new(),
            s_builder: |b| BString::new_in(&b),
        }
        .build()
    }
}

impl Container<&str> for CollectString {
    fn push(&mut self, item: &str) {
        self.with_s_mut(|s| s.push_str(item));
    }
}

impl Output for CollectString {
    fn as_str(&self) -> &str {
        self.with_s(|s| s.as_str())
    }
}

#[inline]
fn parser<'src>() -> impl Parser<'src, &'src str, CollectString, extra::Default> {
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
        .collect::<CollectString>()
}

#[test]
fn test_parse() {
    let s = parse(r#"hello\\\n\tniho\r\"\'\0\bxl\a\n123\b2"#).unwrap();
    println!("{}", s.as_str());
}
