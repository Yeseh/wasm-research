use wasmtime::{
    component::bindgen, component::Component, component::Linker, Config, Engine, Store,
};
use wasmtime_wasi::preview2::{ResourceTable, WasiCtx, WasiView};

bindgen!({
  path: "../../../wit",
  world: "hello",
  async: {
    only_imports: []
  },
});

struct Host {
    table: ResourceTable,
    ctx: wasmtime_wasi::preview2::WasiCtx,
}

impl WasiView for Host {
    fn table(&self) -> &ResourceTable {
        &self.table
    }

    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl HelloImports for Host {
    fn get_name(&mut self) -> wasmtime::Result<String> {
        Ok(String::from("Rust"))
    }

    fn print(&mut self, msg: String) -> wasmtime::Result<()> {
        println!("{}", msg);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    // WASI preview2 requires async support
    config.async_support(true);

    let engine = Engine::new(&config)?;
    let component =
        Component::from_file(&engine, "../../../wasm/components/rust_hello_guest.wasm")?;

    let mut linker = Linker::new(&engine);
    Hello::add_to_linker(&mut linker, |state: &mut Host| state)?;
    wasmtime_wasi::preview2::command::add_to_linker(&mut linker)?;

    let ctx = wasmtime_wasi::preview2::WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_stderr()
        .inherit_stdout()
        .build();

    let mut store = Store::new(
        &engine,
        Host {
            table: ResourceTable::default(),
            ctx,
        },
    );
    let (bindings, _) = Hello::instantiate_async(&mut store, &component, &linker).await?;

    bindings.call_hello(store).await?;

    Ok(())
}
