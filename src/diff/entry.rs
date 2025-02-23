use keepass::Value;
use std::collections::HashMap;

use crate::diff::field::Field;
use crate::diff::{Diff, DiffResult, DiffResultFormat};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub fields: HashMap<String, Field>,
}

impl Entry {
    pub fn from_keepass(e: &keepass::Entry) -> Self {
        // username, password, etc. are just fields
        let fields = e
            .fields
            .iter()
            .map(|(k, v)| {
                (
                    k.to_owned(),
                    Field {
                        name: k.to_owned(),
                        value: match v {
                            Value::Unprotected(v) => v.to_owned(),
                            Value::Protected(p) => String::from_utf8(p.unsecure().to_owned())
                                .unwrap()
                                .to_owned(),
                        },
                    },
                )
            })
            .collect();

        Entry { fields }
    }
}

impl Diff for Entry {
    fn diff<'a>(&'a self, other: &'a Self) -> DiffResult<'a, Self> {
        let (has_differences, field_differences) =
            crate::diff::diff_hashmap(&self.fields, &other.fields);

        if has_differences {
            let mut inner_differences: Vec<Box<dyn DiffResultFormat>> = Vec::new();

            for dr in field_differences {
                inner_differences.push(Box::new(dr))
            }

            DiffResult::InnerDifferences {
                left: self,
                right: other,
                inner_differences,
            }
        } else {
            DiffResult::Identical {
                left: self,
                right: other,
            }
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Entry '{}'", self.fields.get("Title").unwrap().value)
    }
}
