use neon::prelude::*;

fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let octocrab = octocrab::instance();
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
    Ok(JsUndefined::new(&mut cx))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    Ok(())
}
