use proc_macro::*;
use std::iter::Peekable;

#[proc_macro]
pub fn print_stream(input: TokenStream) -> TokenStream {
    for i in input.into_iter() {
        println!("{:?}", i);
    }
    TokenStream::new()
}

#[proc_macro]
pub fn metta(input: TokenStream) -> TokenStream {
    MettaConverter::new(input).run()
}

#[derive(Debug)]
enum Token {
    Space,
    Int(String),
    Float(String),
    Str(String),
    Punct(char),
    Other(TokenTree),
    End,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Self::Space => " ".into(),
            Self::Int(s) => s.clone(),
            Self::Float(s) => s.clone(),
            Self::Str(s) => s.clone(),
            Self::Punct(c) => c.to_string(),
            Self::Other(tt) => tt.to_string(),
            Self::End => unreachable!(),
        }
    }
}

struct Tokenizer {
    prev: (Token, Span),
    input: Peekable<proc_macro::token_stream::IntoIter>,
}

impl Tokenizer {
    fn new(input: TokenStream) -> Self {
        let mut input = input.into_iter().peekable();
        let prev = match input.next() {
            Some(tt) => Self::to_token(tt),
            None => (Token::End, Span::call_site()),
        };
        Self {
            prev,
            input,
        }
    }

    fn next(&mut self) -> Token {

        let mut next = match self.input.peek() {
            None => {
                match self.prev.0 {
                    Token::Space => (Token::End, self.prev.1),
                    _ => (Token::Space, self.prev.1),
                }
            },
            Some(tt) => {
                if Self::range(self.prev.1.end()) == Self::range(tt.span().start()) {
                    Self::to_token(self.input.next().unwrap())
                } else {
                    (Token::Space, tt.span())
                }
            },
        };

        std::mem::swap(&mut next, &mut self.prev);
        next.0
    }

    fn range(span: Span) -> (usize, usize) {
        (span.line(), span.column())
    }

    fn to_token(tt: TokenTree) -> (Token, Span) {
        let span = tt.span();
        let t = match &tt {
            TokenTree::Literal(l) => {
                let s = l.to_string();
                let lit = litrs::Literal::parse(s.clone()).expect("Failed to parse literal");
                match lit {
                    litrs::Literal::Integer(_) => Token::Int(s),
                    litrs::Literal::Float(_) => Token::Float(s),
                    litrs::Literal::String(_) => Token::Str(s),
                    _ => Token::Other(tt),
                }
            },
            TokenTree::Punct(p) => {
                Token::Punct(p.as_char())
            },
            _ => Token::Other(tt),
        };
        (t, span)
    }

}

struct Printer {
    span: Span,
    output: Vec<(Delimiter, TokenStream)>,
}

impl Printer {
    fn new() -> Self {
        Self {
            span: Span::call_site(),
            output: vec![(Delimiter::None, TokenStream::new())],
        }
    }

    fn get_token_stream(&mut self) -> TokenStream {
        assert!(self.output.len() == 1, "Unbalanced group");
        self.output.pop().unwrap().1
    }

    fn push(&mut self, tt: TokenTree) {
        let (_, last) = self.output.last_mut().unwrap();
        last.extend([tt].into_iter());
    }

    fn span(&mut self) -> Span {
        let span = self.span;
        self.span = self.span.end();
        span
    }

    fn ident(&mut self, name: &str) -> &mut Self {
        let span = self.span();
        self.push(TokenTree::Ident(Ident::new(name, span)));
        self
    }

    fn punct(&mut self, chars: &str) -> &mut Self {
        assert!(!chars.is_empty(), "Empty punct");
        let mut chars = chars.chars().peekable();
        let mut c = chars.next().unwrap();
        while chars.peek().is_some()  {
            self.push(TokenTree::Punct(Punct::new(c, Spacing::Joint)));
            c = chars.next().unwrap();
        }
        self.push(TokenTree::Punct(Punct::new(c, Spacing::Alone)));
        self
    }

    fn group(&mut self, d: char) -> &mut Self {
        let (open, delimiter) = match d {
            '(' => (true, Delimiter::Parenthesis),
            '{' => (true, Delimiter::Brace),
            '[' => (true, Delimiter::Bracket),
            ')' => (false, Delimiter::Parenthesis),
            '}' => (false, Delimiter::Brace),
            ']' => (false, Delimiter::Bracket),
            _ => panic!("Unexpected delimiter: {}", d),
        };
        if open {
            self.output.push((delimiter, TokenStream::new()));
        } else {
            assert!(self.output.len() > 1, "Unbalanced group");
            let (d, stream) = self.output.pop().unwrap();
            assert!(d == delimiter, "Closing delimiter {:?} is not equal to opening one {:?}", delimiter, d);
            self.push(TokenTree::Group(Group::new(delimiter, stream)));
        }
        self 
    }

    fn literal(&mut self, lit: Literal) -> &mut Self {
        self.push(TokenTree::Literal(lit));
        self
    }

    fn string(&mut self, text: &str) -> &mut Self {
        self.push(TokenTree::Literal(Literal::string(text)));
        self
    }

    fn symbol(&mut self, name: &str) {
        self.ident("Atom").punct("::").ident("sym").group('(').string(name).group(')');
    }

    fn variable(&mut self, name: &str) {
        self.ident("Atom").punct("::").ident("var").group('(').string(name).group(')');
    }

    fn bool(&mut self, b: &str) {
        self.ident("Atom").punct("::").ident("gnd").group('(').ident("Bool").group('(').ident(b).group(')').group(')');
    }

    fn integer(&mut self, n: i64) {
        self.ident("Atom").punct("::").ident("gnd").group('(')
            .ident("Number").punct("::").ident("Integer").group('(')
            .literal(Literal::i64_suffixed(n)).group(')').group(')');
    }

    fn float(&mut self, f: f64) {
        self.ident("Atom").punct("::").ident("gnd").group('(')
            .ident("Number").punct("::").ident("Float").group('(')
            .literal(Literal::f64_suffixed(f)).group(')').group(')');
    }

    fn str(&mut self, s: &str) {
        self.ident("Atom").punct("::").ident("gnd").group('(')
            .ident("Str").punct("::").ident("from_str").group('(')
            .literal(Literal::string(s)).group(')').group(')');
    }

    fn expr_start(&mut self) {
        self.ident("Atom").punct("::").ident("expr").group('(')
            .group('[');
    }

    fn expr_delimiter(&mut self) {
        self.punct(",");
    }

    fn expr_end(&mut self) {
        self.group(']').group(')');
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Start,
    ExpectSpace,
    VariableName,
    Sign,
    Expression,
    Final,
}

struct MettaConverter {
    state: State,
    buffer: String,
    input: Tokenizer,
    output: Printer,
}

impl MettaConverter {

    fn new(input: TokenStream) -> Self {
        Self{
            state: State::Start,
            buffer: String::new(),
            input: Tokenizer::new(input),
            output: Printer::new(),
        }
    }


    fn run(&mut self) -> TokenStream {
        loop {
            if self.state == State::Final {
                break
            }
            self.next_state();
        }
        self.output.get_token_stream()
    }

    fn next_state(&mut self) {
        let (state, token) = (self.state, self.input.next());
        println!("state: {:?}, token: {:?}", state, token);
        if self.state == State::Start {
            self.buffer.clear();
        }
        self.state = match (state, token) {
            (State::Start, Token::Space) => State::Start,
            (State::Start, Token::End)  => State::Final,
            (State::ExpectSpace, Token::Space) => State::Start,

            (State::Start, Token::Other(TokenTree::Ident(i))) => {
                let ident = i.to_string();
                if ident == "True" || ident == "False" {
                    self.output.bool(&ident.to_lowercase());
                    State::ExpectSpace
                } else {
                    self.output.symbol(&ident);
                    State::ExpectSpace
                }
            },

            (State::Start, Token::Punct(p)) if p == '$' => {
                State::VariableName
            },
            (State::VariableName, Token::Other(tt)) => {
                self.buffer += &tt.to_string();
                State::VariableName
            },
            (State::VariableName, Token::Space) => {
                self.output.variable(&self.buffer);
                State::Start
            },
            (State::VariableName, t) => {
                self.buffer += &t.to_string();
                State::VariableName
            },

            (State::Start, Token::Punct(p)) if p == '-' || p == '+' => {
                self.buffer.push(p);
                State::Sign
            }
            (State::Sign, Token::Space) => {
                self.output.symbol(&self.buffer);
                State::Start
            },
            (State::Sign, Token::Int(s)) => {
                self.buffer += &s;
                self.output.integer(self.buffer.parse::<i64>().unwrap());
                State::Start
            },
            (State::Sign, Token::Float(s)) => {
                self.buffer += &s;
                self.output.float(self.buffer.parse::<f64>().unwrap());
                State::Start
            },

            (State::Start, Token::Int(s)) => {
                self.output.integer(s.parse::<i64>().unwrap());
                State::ExpectSpace
            },

            (State::Start, Token::Float(s)) => {
                self.output.float(s.parse::<f64>().unwrap());
                State::ExpectSpace
            },

            (State::Start, Token::Str(s)) => {
                self.output.str(&s[1..s.len() - 1]);
                State::ExpectSpace
            },

            (State::Start, Token::Other(TokenTree::Group(g)))
                if g.delimiter() == Delimiter::Parenthesis => {
                    self.output.expr_start();
                    State::Expression
            },
            (State::Expression, Token::Space) => {
                State::Expression
            },
            (State::Expression, Token::End) => {
                self.output.expr_end();
                State::Start
            },

            _ => todo!("return error"), 
        }
    }
}
