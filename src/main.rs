use wasmtime::*;

mod wasm_runner;

use wasm_runner::MODULE;

fn main() {
    let store = Store::default();

    let module = Module::new(store.engine(), MODULE).unwrap();

    println!("Instantiating module...");
    let instance = Instance::new(&store, &module, &[]).unwrap();

    println!("Extracting export...");
    let run = instance
        .get_func("add")
        .unwrap()
        .get2::<i32, i32, i32>()
        .unwrap();

    println!("Calling export...");
    let res = run(1, 2).unwrap();

    println!("{}", res);

    println!("Done.");
}
