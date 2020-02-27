use std::fmt;

use crate::Pair;

/// Grammar tree constructed from the parsed input.
#[derive(Debug, PartialEq, Clone)]
pub enum GrammarTree<'i> {
	Atom(&'i str),
	List(Vec<Self>)
}
impl<'i> GrammarTree<'i> {
	/// Formats `self` as if it was top level, skipping the first list parens.
	pub fn fmt_scheme_top_level(&self, f: &mut fmt::Formatter, indent: &str) -> fmt::Result {
		match self {
			GrammarTree::Atom(_) => {
				self.fmt_scheme(f, 0, indent)?;
			}
			GrammarTree::List(list) => list
				.iter()
				.try_for_each(|t| t.fmt_scheme(f, 0, indent).map(|_| ()))?
		};

		Ok(())
	}

	/// Formats `self` as pretty printed scheme code, starting at `depth` indentation level.
	///
	/// Returns `true` if there were any newlines added in the output.
	pub fn fmt_scheme(
		&self,
		f: &mut fmt::Formatter,
		depth: usize,
		indent: &str
	) -> Result<bool, fmt::Error> {
		match self {
			GrammarTree::Atom(atom) => write!(f, "{}", atom).map(|_| false),
			GrammarTree::List(list) => {
				let mut children = list.iter();
				let first = children.next();

				if first.is_none() {
					return write!(f, "()").map(|_| false)
				}
				let first = first.unwrap();

				let do_newline = true;
				if do_newline {
					write!(f, "\n")?;
					for _ in 0 .. depth {
						write!(f, "{}", indent)?;
					}
				}
				write!(f, "(")?;

				let mut any_newline = first.fmt_scheme(f, depth + 1, indent)?;
				for child in children {
					write!(f, " ")?;
					any_newline = child.fmt_scheme(f, depth + 1, indent)? || any_newline;
				}

				if any_newline {
					write!(f, "\n")?;
					for _ in 0 .. depth {
						write!(f, "{}", indent)?;
					}
				}
				write!(f, ")")?;

				Ok(do_newline)
			}
		}
	}
}
impl<'i> fmt::Display for GrammarTree<'i> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.fmt_scheme_top_level(f, "\t")
	}
}
impl<'i> From<Pair<'i>> for GrammarTree<'i> {
	fn from(pair: Pair<'i>) -> Self {
		GrammarTree::Atom(pair.as_str())
	}
}
impl<'i> From<&'i str> for GrammarTree<'i> {
	fn from(string: &'i str) -> Self {
		GrammarTree::Atom(string)
	}
}
impl<'i> From<Vec<Self>> for GrammarTree<'i> {
	fn from(vec: Vec<Self>) -> Self {
		GrammarTree::List(vec)
	}
}
