use nom::*;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Primitive(Prim),
    SpecialForm(SForm),
    User(String),
}

#[derive(Debug, PartialEq, Eq)]
enum Prim {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug, PartialEq, Eq)]
enum SForm {
    If,
    Begin,
    Define,
    Lambda,
    Let,
}

named!(string<&[u8], String>,
do_parse!(
    word: ws!(alphanumeric) >>
    (String::from_utf8(word.to_vec()).unwrap())
)
);

named!(specialform<&[u8], Op>,
alt!(
    map!(tag!("if"), |_| Op::SpecialForm(SForm::If)) |
    map!(tag!("begin"), |_| Op::SpecialForm(SForm::Begin)) |
    map!(tag!("define"), |_| Op::SpecialForm(SForm::Define)) |
    map!(tag!("lambda"), |_| Op::SpecialForm(SForm::Lambda)) |
    map!(tag!("let"), |_| Op::SpecialForm(SForm::Let))
)
);
// https://blog.mgattozzi.dev/scheme-parser/
named!(primitive<&[u8], Op>,
alt!(
    map!(tag!("*"), |_| Op::Primitive(Prim::Mul)) |
    map!(tag!("+"), |_| Op::Primitive(Prim::Add)) |
    map!(tag!("-"), |_| Op::Primitive(Prim::Sub)) |
    map!(tag!("/"), |_| Op::Primitive(Prim::Div))
)
);

named!(user<&[u8], Op>,
do_parse!(
    user_proc: string >>
    (Op::User(user_proc))
)
);

#[test]
fn string_parser() {
    let comp_string = String::from("hi");

    match string(b"hi") {
        IResult::Done(_, s) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string"),
    }
    match string(b"    hi    ") {
        IResult::Done(_, s) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string"),
    }
    match string(b"hi     ") {
        IResult::Done(_, s) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string"),
    }
    match string(b"       hi") {
        IResult::Done(_, s) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string"),
    }
}
