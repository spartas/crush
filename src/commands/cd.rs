use crate::commands::CompileContext;
use crate::errors::JobResult;
use crate::{
    data::Argument,
    commands::Exec,
    errors::JobError,
    env::Env,
    errors::to_job_error,
    printer::Printer,
    stream::{OutputStream, InputStream},
    data::ColumnType
};
use std::path::Path;
use crate::data::Cell;

pub fn run(dir: Box<Path>) -> JobResult<()> {
    to_job_error(std::env::set_current_dir(dir))
}

pub fn compile(context: CompileContext) -> JobResult<(Exec, Vec<ColumnType>)> {
    let dir = match context.arguments.len() {
        0 => Ok(Box::from(Path::new("/"))),
        1 => {
            let dir = &context.arguments[0];
            match &dir.cell {
                Cell::Text(val) => Ok(Box::from(Path::new(val.as_ref()))),
                Cell::File(val) => Ok(val.clone()),
                _ => Err(JobError { message: String::from("Wrong parameter type, expected text") })
            }
        }
        _ => Err(JobError { message: String::from("Wrong number of arguments") })
    }?;
    Ok((Exec::Command(Box::from(move || run(dir))), vec![]))
}
