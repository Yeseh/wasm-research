wit_bindgen::generate!({
  path: "../../../wit",
  world: "hello",
  exports: { world: Hello }
});

struct Hello;

impl Guest for Hello {
    fn hello() {
        let name = get_name();
        let msg = format!("Hello, {}!", name);
        print(msg.as_str());
    }
}
