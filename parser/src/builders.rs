//! Parser factory functions (builders).
//!
//! Much of the code in this module was inspired by the [nom](https://github.com/Geal/nom/)
//! library.  One or two particular parts of this module were copied more or
//! less directly from `nom`.  Therefore, the `nom` license is included below
//! and will apply where explicitly noted.  Otherwise, though the API of this
//! module was directly taken from `nom`, the actual implementations of most
//! parser builders are original, though they may coincide with the
//! implementations of nom parser builders.
//!
//! ```text
//! <<<< BEGIN NOM LICENSE >>>>
//!
//! Copyright (c) 2014-2019 Geoffroy Couprie
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to
//! deal in the Software without restriction, including without limitation the
//! rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
//! sell copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in
//! all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
//! FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
//! IN THE SOFTWARE.
//!
//! <<<< END NOM LICENSE >>>>
//! ```
//!
//! The above license applies to any component enclosed in a pair of comments as
//! follows:
//!
//! ```text
//! // <<< BEGIN NOM LICENSE COVERAGE >>>
//!
//! ...
//!
//! // <<< END NOM LICENSE COVERAGE >>>
//! ```

use crate::ast::Expr;
use crate::errors::{
    ErrorKind,
    ParseError,
};
use crate::node::{
    Node,
    Span,
};
use crate::{
    Cursor,
    ParseResult,
};

// <<<< BEGIN NOM LICENSE COVERAGE >>>>
macro_rules! succ {
  (0, $macro:ident ! ($($args:tt)*)) => ($macro!(1, $($args)*));
  (1, $macro:ident ! ($($args:tt)*)) => ($macro!(2, $($args)*));
  (2, $macro:ident ! ($($args:tt)*)) => ($macro!(3, $($args)*));
  (3, $macro:ident ! ($($args:tt)*)) => ($macro!(4, $($args)*));
  (4, $macro:ident ! ($($args:tt)*)) => ($macro!(5, $($args)*));
  (5, $macro:ident ! ($($args:tt)*)) => ($macro!(6, $($args)*));
  (6, $macro:ident ! ($($args:tt)*)) => ($macro!(7, $($args)*));
  (7, $macro:ident ! ($($args:tt)*)) => ($macro!(8, $($args)*));
  (8, $macro:ident ! ($($args:tt)*)) => ($macro!(9, $($args)*));
  (9, $macro:ident ! ($($args:tt)*)) => ($macro!(10, $($args)*));
  (10, $macro:ident ! ($($args:tt)*)) => ($macro!(11, $($args)*));
  (11, $macro:ident ! ($($args:tt)*)) => ($macro!(12, $($args)*));
  (12, $macro:ident ! ($($args:tt)*)) => ($macro!(13, $($args)*));
  (13, $macro:ident ! ($($args:tt)*)) => ($macro!(14, $($args)*));
  (14, $macro:ident ! ($($args:tt)*)) => ($macro!(15, $($args)*));
  (15, $macro:ident ! ($($args:tt)*)) => ($macro!(16, $($args)*));
  (16, $macro:ident ! ($($args:tt)*)) => ($macro!(17, $($args)*));
  (17, $macro:ident ! ($($args:tt)*)) => ($macro!(18, $($args)*));
  (18, $macro:ident ! ($($args:tt)*)) => ($macro!(19, $($args)*));
  (19, $macro:ident ! ($($args:tt)*)) => ($macro!(20, $($args)*));
}

pub trait Alt<'a, Output> {
    fn parse(&self, input: Cursor<'a>) -> ParseResult<'a, Output>;
}

macro_rules! alt_trait_impl {
    ($($type_var:ident)+) => {
        impl<'a, Output, $($type_var),+> Alt<'a, Output> for ($($type_var),+)
        where
            $($type_var: Fn(Cursor<'a>) -> ParseResult<Output>),+
        {
            fn parse(&self, input: Cursor<'a>) -> ParseResult<'a, Output> {
                alt_parse_body!(0, self, input, pos, $($type_var)+)
            }
        }
    };
}

macro_rules! alt_parse_body {
    ($id:tt, $self:expr, $input:expr, $pos:expr, $head:ident $($tail:ident)+) => {{
        let result = $self.$id($input);
        if result.is_ok() {
            return result;
        }
        succ!($id, alt_parse_body!($self, $input, $pos, $($tail)+))
    }};
    ($id:tt, $self:expr, $input:expr, $pos:expr, $head:ident) => {{
        $self.$id($input)
    }};
}

alt_trait_impl!(A B);
alt_trait_impl!(A B C);
alt_trait_impl!(A B C D);
alt_trait_impl!(A B C D E);
alt_trait_impl!(A B C D E F);
alt_trait_impl!(A B C D E F G);
alt_trait_impl!(A B C D E F G H);
alt_trait_impl!(A B C D E F G H I);
alt_trait_impl!(A B C D E F G H I J);
alt_trait_impl!(A B C D E F G H I J K);
alt_trait_impl!(A B C D E F G H I J K L);
alt_trait_impl!(A B C D E F G H I J K L M);
alt_trait_impl!(A B C D E F G H I J K L M N);
alt_trait_impl!(A B C D E F G H I J K L M N O);
alt_trait_impl!(A B C D E F G H I J K L M N O P);
alt_trait_impl!(A B C D E F G H I J K L M N O P Q);
alt_trait_impl!(A B C D E F G H I J K L M N O P Q R);
alt_trait_impl!(A B C D E F G H I J K L M N O P Q R S);
alt_trait_impl!(A B C D E F G H I J K L M N O P Q R S T);
alt_trait_impl!(A B C D E F G H I J K L M N O P Q R S T U);

pub fn alt<'a, O, A: Alt<'a, O>>(alts: A) -> impl Fn(Cursor<'a>) -> ParseResult<O> {
    move |input| alts.parse(input)
}
// <<<< END NOM LICENSE COVERAGE >>>>

pub fn many1<'a, O, P>(parser: P) -> impl Fn(Cursor<'a>) -> ParseResult<Vec<O>>
where
    P: Fn(Cursor<'a>) -> ParseResult<O>,
{
    move |input| match parser(input) {
        Ok((input, first)) => {
            let mut input = input;
            let mut results = vec![first];

            while let Ok((next_input, next)) = parser(input) {
                input = next_input;
                results.push(next);
            }

            Ok((input, results))
        }
        Err(err) => Err(err.push(
            input,
            ErrorKind::StaticStr("many1: expected at least one occurrence"),
        )),
    }
}

pub fn many0<'a, O, P>(parser: P) -> impl Fn(Cursor<'a>) -> ParseResult<Vec<O>>
where
    P: Fn(Cursor<'a>) -> ParseResult<O>,
{
    move |input| {
        if input.is_empty() {
            return Err(ParseError::eof(input));
        }

        let mut input = input;
        let mut results = vec![];

        while let Ok((next_input, next)) = parser(input) {
            input = next_input;
            results.push(next);
        }

        Ok((input, results))
    }
}

// <<<< BEGIN NOM LICENSE COVERAGE >>>>
pub fn preceded<'a, O1, O2, F, G>(f: F, g: G) -> impl Fn(Cursor<'a>) -> ParseResult<O2>
where
    F: Fn(Cursor<'a>) -> ParseResult<O1>,
    G: Fn(Cursor<'a>) -> ParseResult<O2>,
{
    move |input| {
        let (input, _) = f(input)?;
        g(input)
    }
}
// <<<< END NOM LICENSE COVERAGE >>>>

pub fn terminated<'a, O1, O2, F, G>(f: F, g: G) -> impl Fn(Cursor<'a>) -> ParseResult<O1>
where
    F: Fn(Cursor<'a>) -> ParseResult<O1>,
    G: Fn(Cursor<'a>) -> ParseResult<O2>,
{
    move |input| {
        let (input, result) = f(input)?;
        let (input, _) = g(input)?;
        Ok((input, result))
    }
}

pub fn pair<'a, O1, O2, F, G>(f: F, g: G) -> impl Fn(Cursor<'a>) -> ParseResult<(O1, O2)>
where
    F: Fn(Cursor<'a>) -> ParseResult<O1>,
    G: Fn(Cursor<'a>) -> ParseResult<O2>,
{
    move |input| {
        let (input, result_1) = f(input)?;
        let (input, result_2) = g(input)?;
        Ok((input, (result_1, result_2)))
    }
}

pub fn opt<'a, O, P>(parser: P) -> impl Fn(Cursor<'a>) -> ParseResult<Option<O>>
where
    P: Fn(Cursor<'a>) -> ParseResult<O>,
{
    move |input| match parser(input) {
        Ok((input_ok, result)) => Ok((input_ok, Some(result))),
        Err(_) => Ok((input, None)),
    }
}

pub fn map<'a, O1, O2, P, M>(parser: P, mapper: M) -> impl Fn(Cursor<'a>) -> ParseResult<O2>
where
    P: Fn(Cursor<'a>) -> ParseResult<O1>,
    M: Fn(O1) -> O2,
{
    move |input| {
        let (input, result) = parser(input)?;
        Ok((input, mapper(result)))
    }
}

pub fn verify<'a, O, P, V, D>(
    parser: P,
    verifier: V,
    describer: D,
) -> impl Fn(Cursor<'a>) -> ParseResult<O>
where
    P: Fn(Cursor<'a>) -> ParseResult<O>,
    V: Fn(&O) -> bool,
    D: Fn(Cursor<'a>, &O) -> ParseError<'a>,
{
    move |input| {
        let orig_input = input;
        let (input, result) = parser(input)?;

        if verifier(&result) {
            Ok((input, result))
        } else {
            Err(describer(orig_input, &result))
        }
    }
}

/// Modify a parser to apply itself repeatedly while consuming separators
/// (including an optional trailing separator).
pub fn separated<'a, P, S, O1, O2>(
    parser: P,
    sep: S,
    parse_trailing: bool,
) -> impl Fn(Cursor<'a>) -> ParseResult<Vec<O1>>
where
    P: Fn(Cursor<'a>) -> ParseResult<O1>,
    S: Fn(Cursor<'a>) -> ParseResult<O2>,
{
    move |input| {
        let (input, first) = parser(input)?;
        let (input, mut rest) = many0(preceded(&sep, &parser))(input)?;

        let mut results = vec![first];
        results.append(&mut rest);

        let input = if parse_trailing {
            match opt(&sep)(input) {
                Ok((input_, _)) => input_,
                _ => input,
            }
        } else {
            input
        };

        Ok((input, results))
    }
}

/// Modify a parser to parse and discard left and right delimiters before and
/// after the parsed content.  The parsed delimiters are used to determine the
/// span of the parsing result.
pub fn delimited<'a, L, P, R, O1, O2, O3>(
    left: L,
    parser: P,
    right: R,
) -> impl Fn(Cursor<'a>) -> ParseResult<Node<O2>>
where
    L: Fn(Cursor<'a>) -> ParseResult<O1>,
    P: Fn(Cursor<'a>) -> ParseResult<O2>,
    R: Fn(Cursor<'a>) -> ParseResult<O3>,
    O1: Into<Span>,
    O3: Into<Span>,
{
    move |input| {
        let (input, l_delim) = left(input)?;
        let (input, node) = parser(input)?;
        let (input, r_delim) = right(input)?;

        Ok((
            input,
            Node::new(node, Span::from_pair(l_delim.into(), r_delim.into())),
        ))
    }
}

pub fn op_expr_builder<'a, F, G, B, OperatorT>(
    operand: F,
    operator: G,
    builder: B,
) -> impl Fn(Cursor<'a>) -> ParseResult<Node<Expr>>
where
    F: Fn(Cursor<'a>) -> ParseResult<Node<Expr>>,
    G: Fn(Cursor<'a>) -> ParseResult<OperatorT>,
    B: Fn(Node<Expr>, OperatorT, Node<Expr>) -> Expr,
{
    move |input| {
        let (input, head) = operand(input)?;
        let (input, tail) = many0(pair(&operator, &operand))(input)?;

        let mut left = head;
        for (oprtr, right) in tail {
            let span = Span::from_pair(&left, &right);

            left = Node::new(builder(left, oprtr, right), span);
        }

        Ok((input, left))
    }
}
