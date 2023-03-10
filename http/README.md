# Rust WASI HTTP Function

🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉
## EXPERIMENTAL - HERE BE DRAGONS
🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉

## TODO LIST

* [x] HTTP Handler

* [ ] Build Manifest

* [x] Readiness Probe

* [x] Liveness Probe

* [ ] func instructions

A knative func template project for WASI and Cloud Events

Welcome to your new Rust WASI function project! The boilerplate web server is in
[`src/main.rs`](./src/main.rs). It's configured to invoke the `index`
function in [`src/handler.rs`](./src/handler.rs) in response to both
GET and POST requests. You should put your desired behavior inside
that `index` function.

In case you need to configure some resources for your function, you can do that in the [`configure` function](./src/config.rs).

The app will expose three endpoints:

  * `/` Triggers the `index` function, for either GET or POST methods
  * `/health/readiness` The endpoint for a readiness health check **Not Available**
  * `/health/liveness` The endpoint for a liveness health check **Not Available**

## Development

To get started you will need the following

1. [install WASMEdge](https://wasmedge.org/book/en/quick_start/install.html)

2. Install `cargo wasi` with `cargo install cargo-wasi`

3. Set `CARGO_TARGET_WASM32_WASI_RUNNER=wasmedge` in your `.profile`.


Once the setup is complete you should be able to run the following commands successfully
```shell script
cargo wasi build # build your code in debug mode for the wasi target.

cargo wasi build --release # build the optimized version of your *.wasm.

cargo wasi run # execute a binary.

cargo wasi test # run your tests in wasm32-wasi.
# OR
cargo test --target wasm32-wasi -- --nocapture # If you want more verbose output

cargo wasi bench # run your benchmarks in wasm32-wasi.

cargo clippy --target wasm32-wasi # There is no wasi wrapper for clippy as yet
```

Once running, the function is available at <http://localhost:8080> and
the health checks are at <http://localhost:8080/health/readiness> and
<http://localhost:8080/health/liveness>. To POST data to the function,
a utility such as `curl` may be used:

```console
curl -d '{"hello": "world"}' \
  -H'content-type: application/json' \
  http://localhost:8080
```

## Deployment

⚠️ Not yet implemented ⚠️

Should follow the standard func pattern

```shell script
func deploy --registry=docker.io/<YOUR_ACCOUNT>
```

<!--
Use `func` to containerize your application, publish it to a registry
and deploy it as a Knative Service in your Kubernetes cluster:

```shell script
func deploy --registry=docker.io/<YOUR_ACCOUNT>
```

You can omit the `--registry` option by setting the `FUNC_REGISTRY`
environment variable. And if you forget, you'll be prompted.

The output from a successful deploy should show the URL for the
service, which you can also get via `func info`, e.g.

```console
curl $(func info -o url)
```
-->
Have fun!
