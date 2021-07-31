use neon::prelude::*;

fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    println!("howdy from rust");
    Ok(JsUndefined::new(&mut cx))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    Ok(())
}
