use std::collections::{hash_map::Entry, HashMap};

use super::value::{LValue, RValue};

#[derive(Default)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    vars: HashMap<String, Option<RValue>>,
}

impl Environment {
    pub fn new_scope(self) -> Self {
        Self {
            parent: Some(Box::new(self)),
            ..Default::default()
        }
    }

    pub fn end_scope(self) -> Option<Self> {
        Some(*self.parent?)
    }

    pub fn new_var(&mut self, name: LValue, val: Option<RValue>) {
        match self.vars.entry(name) {
            Entry::Occupied(o) => panic!(
                "variable '{}' already exists in this scope. you cannot assign values with var",
                o.key()
            ),
            Entry::Vacant(v) => {
                v.insert(val);
            }
        }
    }

    pub fn set_var(&mut self, name: LValue, val: RValue) {
        match self.get_entry(name) {
            Entry::Vacant(v) => panic!("unable to assign '{}'. variable does not exist", v.key()),
            Entry::Occupied(mut o) => o.insert(Some(val)),
        };
    }

    pub fn get_var(&self, name: &LValue) -> &RValue {
        self.vars
            .get(name)
            .map_or_else(
                || match &self.parent {
                    Some(p) => Some(p.get_var(name)),
                    None => panic!("variable '{name}' does not exist"),
                },
                |x| x.as_ref(),
            )
            .unwrap_or(&RValue::Null)
    }

    fn get_entry(&mut self, name: LValue) -> Entry<LValue, Option<RValue>> {
        match self.vars.entry(name) {
            Entry::Vacant(v) if self.parent.is_some() => {
                self.parent.as_mut().unwrap().get_entry(v.into_key())
            }
            e => e,
        }
    }
}
