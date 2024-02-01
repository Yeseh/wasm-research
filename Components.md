# WebAssembly components
TODO: describe what components are, how to use them, and how to compose them into applications.

## Virtual Platform Layering
- A pattern of composing wasm components 
- Run a component in a different world 
- World: what do I import from the environment and what do I expose.
- For client components: what do I need to do my work
- For host components: what do I need to provide to my clients?
- Some classes of worlds:
    - wasi-cli-world
    - wasi-cloud-world
    - standardized collection of functionality, based on the platform that have certain definitions
    - Worlds can include eachother, extend standardized worlds by including your own stuff.
- Virtuality: components all the way down, doesn't have to be implemented in the same environment as 
- What is a component?
    - WASMs version of a container
    - Container does all of linux
    - Components are more portable to hardware and OS
    - Feels like modules and packages
    - Language agnostic


### Examples
- Source written for POSIX env, wasi-cli world. How do we run it in a non POSIX env such as a cloud service backed by a blob store instead of a filesystem?
- Local testing, run with a database in production, against filesystem locally
- Building platform for customers, plugin system 
- Platform-on-platform action, ex. heroku on Amazon
