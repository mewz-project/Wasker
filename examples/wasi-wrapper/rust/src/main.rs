mod memory;
mod wasi;

extern "C" {
    fn wasker_main();
}

fn main() {
    unsafe {
        // Entrypoint of ELF generated by Wasker
        wasker_main();
    }
    println!("wasker_main from the compiled target WASM done.");
}