use std::{borrow::Cow, fmt, sync::Arc};
/// A term.
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Term {
    /// A variable (e.g. `X0`, `Y1`, etc).
    Var(Variable),
    /// A constant (atom or number).
    Const(Constant),
    /// A compound term (`ident(args...)`).
    Application(Ident, Vec<Term>),
}

// TODO(eliza): intern atoms?
pub type Ident = Arc<Cow<'static, str>>;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Constant {
    Num(i64),
    Ident(Ident),
}

/// Variables are strings starting with upper-case letters, followed by
/// a number which indicates an instance of the variable.
///
/// Thus, a variable instance is a pair `[(x,n)]` where `[x]` is a variable and `[n]` is an
/// integer. When the proof search depth is `[n]` all variables that we need to use
/// are renamed from `[(x,0)]` to `[(x,n)]`. This is necessary so that we do not use
/// the same variable name in two different applications of the same assertion.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Variable {
    name: Ident,
    depth: usize,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Atom {}

// === impl Term ===

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(v) => v.fmt(f),
            Term::Const(c) => c.fmt(f),
            Term::Application(ident, args) => {
                write!(f, "{ident}(")?;
                let mut args = args.iter();
                if let Some(arg) = args.next() {
                    arg.fmt(f)?;
                    for arg in args {
                        write!(f, ", {arg}")?;
                    }
                }
                f.write_str(")")
            }
        }
    }
}

// === impl Constant ===

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(n) => n.fmt(f),
            Self::Ident(i) => i.fmt(f),
        }
    }
}

// === impl Variable ===

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name, depth } = self;
        write!(f, "{name}{depth}")
    }
}
