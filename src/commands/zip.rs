use crate::commands::CompileContext;
use crate::errors::CrushResult;
use crate::errors::error;
use crate::data::Value;
use crate::stream::{ValueSender};
use crate::stream::Readable;

pub fn run(input1: &mut impl Readable, input2: &mut impl Readable, sender: ValueSender) -> CrushResult<()> {
    let mut output_type = Vec::new();
    output_type.append(&mut input1.get_type().clone());
    output_type.append(&mut input2.get_type().clone());
    let output = sender.initialize(output_type)?;
    loop {
        match (input1.read(), input2.read()) {
            (Ok(mut row1), Ok(mut row2)) => {
                row1.append(&mut row2.into_vec());
                output.send(row1)?;
            }
            _ => break,
        }
    }
    return Ok(());
}

pub fn perform(mut context: CompileContext) -> CrushResult<()> {
    if context.arguments.len() != 2 {
        return Err(error("Expected exactly two arguments"));
    }
    match (context.arguments.remove(0).value, context.arguments.remove(0).value) {
        (Value::Stream(mut o1), Value::Stream(mut o2)) =>
            run(&mut o1.reader(), &mut o2.reader(), context.output),
        (Value::Rows(mut o1), Value::Rows(mut o2)) =>
            run(&mut o1.reader(), &mut o2.reader(), context.output),
        (Value::Stream(mut o1), Value::Rows(mut o2)) =>
            run(&mut o1.reader(), &mut o2.reader(), context.output),
        (Value::Rows(mut o1), Value::Stream(mut o2)) =>
            run(&mut o1.reader(), &mut o2.reader(), context.output),
        _ => return Err(error("Expected two datasets")),
    }
}