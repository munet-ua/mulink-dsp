use clap::Parser;
use mulink_dsp::programs::ir_extract::ir_extract;
use mulink_dsp::programs::ir_extract::IrExtractArgs;


pub fn main() -> anyhow::Result<()> {
    ir_extract(IrExtractArgs::parse())
}