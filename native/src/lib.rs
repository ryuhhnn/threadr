#[macro_use]
extern crate neon;

use neon::prelude::*;
use std::process::Command;
use std::sync::mpsc;
use std::thread;

fn spawn(mut cx: FunctionContext) -> JsResult<JsString> {
    // Create message channel
    let (tx, rx) = mpsc::channel();

    // Get the command path
    let cmd = cx.argument::<JsString>(0)?.value();

    // Spawn a thread to run command
    thread::spawn(move || {
        let output = Command::new(cmd)
            .output()
            .expect("failed to execute process");
        tx.send(output).unwrap();
    });

    // Print output for now
    match rx.recv() {
        Ok(output) => println!("output: {:?}", output),
        Err(err) => panic!("error: {:?}", err),
    }

    // Return success string
    Ok(cx.string("success"))
}

register_module!(mut cx, { cx.export_function("spawn", spawn) });
