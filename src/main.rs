use std::io::{self, Write};
use std::path::Path;

use ndarray::{array, concatenate, s, Array1, Axis, CowArray};
use ort::{download::language::machine_comprehension::GPT2, tensor::OrtOwnedTensor, Environment, ExecutionProvider, GraphOptimizationLevel, OrtResult, SessionBuilder, Value, LoggingLevel};
use rand::Rng;
use tokenizers::models::bpe::BPE;
use tokenizers::Tokenizer;

const PROMPT: &str = "一个程序员。热爱分享、创建与开源，努力去改变自己，改变世界。";
const GEN_TOKENS: i32 = 90;
const TOP_K: usize = 5;
fn main() -> OrtResult<()> {
    tracing_subscriber::fmt::init();

    let model_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("quantized_bert-tiny-onnx");

    let mut stdout = io::stdout();
    let mut rng = rand::thread_rng();

    let environment = Environment::builder()
        .with_name("Encode")
        .with_log_level(LoggingLevel::Warning)
        .with_execution_providers([ExecutionProvider::CPU(Default::default())])
        .build()?
        .into_arc();

    let session = SessionBuilder::new(&environment)?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(1)?
        .with_model_from_file(model_dir.join("model_quantized.onnx"))
        .unwrap();

    // let tokenizer = Tokenizer::from_file(model_dir.join("vocab.json")).unwrap();
    // let tokens = tokenizer.encode(PROMPT, false).unwrap();
    // let tokens = tokens.get_ids().iter().map(|i| *i as i64).collect::<Vec<_>>();

    // let mut tokens = CowArray::from(Array1::from_iter(tokens.iter().cloned()));
    // print!("{PROMPT}");
    // stdout.flush().unwrap();


    Ok(())

}