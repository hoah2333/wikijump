/*
 * tree/element/iter_owned.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::{Element, Elements};

impl<'t> IntoIterator for Elements<'t> {
    type Item = Element<'t>;
    type IntoIter = OwnedElementsIterator<'t>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Elements::None => OwnedElementsIterator::None,
            Elements::Single(element) => OwnedElementsIterator::Single(Box::new(Some(element))),
            Elements::Multiple(mut elements) => {
                // So we can just pop for each step
                elements.reverse();
                OwnedElementsIterator::Multiple(elements)
            }
        }
    }
}

/// Owned iterator implementation for `Elements`.
#[derive(Debug)]
pub enum OwnedElementsIterator<'t> {
    None,
    Single(Box<Option<Element<'t>>>),
    Multiple(Vec<Element<'t>>),
}

impl<'t> Iterator for OwnedElementsIterator<'t> {
    type Item = Element<'t>;

    #[inline]
    fn next(&mut self) -> Option<Element<'t>> {
        match self {
            OwnedElementsIterator::None => None,
            OwnedElementsIterator::Single(ref mut element) => element.take(),
            OwnedElementsIterator::Multiple(ref mut elements) => elements.pop(),
        }
    }
}

#[test]
fn iter() {
    macro_rules! check {
        ($elements:expr, $expected:expr $(,)?) => {{
            let elements = $elements;

            let actual: Vec<Element> = elements.into_iter().collect();
            let expected = $expected;

            assert_eq!(
                actual, expected,
                "Actual element iteration doesn't match expected",
            );
        }};
    }

    check!(Elements::None, vec![]);
    check!(Elements::Single(text!("a")), vec![text!("a")]);
    check!(
        Elements::Multiple(vec![]), //
        vec![],
    );
    check!(
        Elements::Multiple(vec![text!("a")]), //
        vec![text!("a")],
    );
    check!(
        Elements::Multiple(vec![text!("a"), text!("b")]),
        vec![text!("a"), text!("b")],
    );
    check!(
        Elements::Multiple(vec![text!("a"), text!("b"), text!("c")]),
        vec![text!("a"), text!("b"), text!("c")],
    );
}