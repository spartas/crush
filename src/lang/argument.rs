use crate::lang::value::Value;
use crate::lang::{value::ValueDefinition, table::ColumnType};
use crate::lang::errors::{CrushError, CrushResult};
use crate::lang::scope::Scope;
use crate::lang::printer::Printer;
use crate::lang::job::JobJoinHandle;

#[derive(Debug, Clone)]
pub struct BaseArgument<C> {
    pub name: Option<Box<str>>,
    pub value: C,
}

pub type ArgumentDefinition = BaseArgument<ValueDefinition>;

impl ArgumentDefinition {
    pub fn argument(&self, dependencies: &mut Vec<JobJoinHandle>, env: &Scope, printer: &Printer) -> Result<Argument, CrushError> {
        Ok(Argument { name: self.name.clone(), value: self.value.compile(dependencies, env, printer)? })
    }
}
/*
impl Clone for ArgumentDefinition {
    fn clone(&self) -> Self {
        ArgumentDefinition { name: self.name.clone(), value: self.value.clone() }
    }
}
*/
pub type Argument = BaseArgument<Value>;

impl Argument {
    pub fn cell_type(&self) -> ColumnType {
        ColumnType { name: self.name.clone(), cell_type: self.value.value_type() }
    }
}

impl<C> BaseArgument<C> {
    pub fn new(name: Option<Box<str>>, value: C) -> BaseArgument<C> {
        BaseArgument {
            name,
            value,
        }
    }

    pub fn named(name: &str, value: C) -> BaseArgument<C> {
        BaseArgument {
            name: Some(Box::from(name)),
            value,
        }
    }

    pub fn unnamed(value: C) -> BaseArgument<C> {
        BaseArgument {
            name: None,
            value,
        }
    }

    pub fn val_or_empty(&self) -> &str {
        self.name.as_ref().map(|v| v.as_ref()).unwrap_or("")
    }
}

pub trait ArgumentVecCompiler {
    fn compile(&self, dependencies: &mut Vec<JobJoinHandle>, env: &Scope, printer: &Printer) -> CrushResult<Vec<Argument>>;
}

impl ArgumentVecCompiler for Vec<ArgumentDefinition> {
    fn compile(&self, dependencies: &mut Vec<JobJoinHandle>, env: &Scope, printer: &Printer) -> CrushResult<Vec<Argument>> {
        self.iter()
            .map(|a| a.argument(dependencies, env, printer))
            .collect::<CrushResult<Vec<Argument>>>()
    }
}