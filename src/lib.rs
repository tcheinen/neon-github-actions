use neon::prelude::*;

fn init(mut cx: FunctionContext) -> JsResult<JsString> {
    println!("howdy from rust");
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", init)?;
    Ok(())
}
