use crate::commands::PerItemCommand;
use crate::parser::registry;
use crate::prelude::*;
use nu_protocol::{
    CallInfo, ReturnSuccess, Scope, Signature, SyntaxShape, UntaggedValue, Value,
};
use nu_errors::ShellError;

pub struct Where;

impl PerItemCommand for Where {
    fn name(&self) -> &str {
        "where"
    }

    fn signature(&self) -> Signature {
        Signature::build("where").required(
            "condition",
            SyntaxShape::Block,
            "the condition that must match",
        )
    }

    fn usage(&self) -> &str {
        "Filter table to match the condition."
    }

    fn run(
        &self,
        call_info: &CallInfo,
        _registry: &registry::CommandRegistry,
        _raw_args: &RawCommandArgs,
        input: Value,
    ) -> Result<OutputStream, ShellError> {
        let input_clone = input.clone();
        let condition = call_info.args.expect_nth(0)?;
        let stream = match condition {
            Value {
                value: UntaggedValue::Block(block),
                ..
            } => {
                let result = block.invoke(&Scope::new(input_clone.clone()));
                match result {
                    Ok(v) => {
                        if v.is_true() {
                            VecDeque::from(vec![Ok(ReturnSuccess::Value(input_clone))])
                        } else {
                            VecDeque::new()
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Value { tag, .. } => {
                return Err(ShellError::labeled_error(
                    "Expected a condition",
                    "where needs a condition",
                    tag,
                ))
            }
        };

        Ok(stream.to_output_stream())
    }
}
