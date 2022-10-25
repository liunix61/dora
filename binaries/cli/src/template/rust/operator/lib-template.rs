use dora_operator_api::{register_operator, DoraOperator, DoraOutputSender, DoraStatus};
use std::time::{Duration, Instant};

register_operator!(ExampleOperator);

#[derive(Debug, Default)]
struct ExampleOperator {
    example_field: u32,
}

impl DoraOperator for ExampleOperator {
    fn on_input(
        &mut self,
        id: &str,
        data: &[u8],
        output_sender: &mut DoraOutputSender,
    ) -> Result<DoraStatus, String> {
        match id {
            other => eprintln!("ignoring unexpected input {other}"),
        }
        Ok(DoraStatus::Continue)
    }
}