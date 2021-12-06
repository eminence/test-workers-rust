This is a temporarily repo to share some worker-rs code that is giving me problems.  See also:
https://github.com/cloudflare/workers-rs/issues/94

# Version info
rustc 1.58.0-nightly (936238a92 2021-11-11)\
wrangler 1.19.5

# Reproduction transcripts:

Build:

```
$ wrangler build
🌀  Running cargo install -q worker-build && worker-build --release
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
    Finished release [optimized] target(s) in 0.28s
[INFO]: Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 7.63s
[INFO]: :-) Your wasm pkg is ready to publish at /nas/achin/devel/id_counter_cfw/build.
✨  Build completed successfully!
```

Run:

```
$ wrangler dev
🌀  Running cargo install -q worker-build && worker-build --release
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
    Finished release [optimized] target(s) in 0.25s
[INFO]: Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 8.38s
[INFO]: :-) Your wasm pkg is ready to publish at /nas/achin/devel/id_counter_cfw/build.
👂  Listening on http://127.0.0.1:8787
```


Test with curl:

    curl http://127.0.0.1:8787

(Curl returned an HTML error page, but I'm not reproducing it here)


Complete output from the `wrangler dev` terminal:

```
$ wrangler dev
🌀  Running cargo install -q worker-build && worker-build --release
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
    Finished release [optimized] target(s) in 0.25s
[INFO]: Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 8.38s
[INFO]: :-) Your wasm pkg is ready to publish at /nas/achin/devel/id_counter_cfw/build.
👂  Listening on http://127.0.0.1:8787
TypeError: Fetch API cannot load: /test
panicked at 'TypeError: Fetch API cannot load: /test', src/lib.rs:43:1

Stack:

Error
    at __wbg_new_693216e109162396 (./index_bg.mjs:368:15)
    at wasm://wasm/00194036:wasm-function[156]:0x1ff5c
    at wasm://wasm/00194036:wasm-function[596]:0x2b3b1
    at wasm://wasm/00194036:wasm-function[211]:0x23c5b
    at wasm://wasm/00194036:wasm-function[290]:0x26f3d
    at wasm://wasm/00194036:wasm-function[439]:0x2a387
    at wasm://wasm/00194036:wasm-function[466]:0x2a899
    at wasm://wasm/00194036:wasm-function[422]:0x29f0f
    at wasm://wasm/00194036:wasm-function[116]:0x1c1ee
    at wasm://wasm/00194036:wasm-function[268]:0x26321


RuntimeError: unreachable
    at wasm://wasm/00194036:wasm-function[211]:0x23c91
    at wasm://wasm/00194036:wasm-function[290]:0x26f3d
    at wasm://wasm/00194036:wasm-function[439]:0x2a387
    at wasm://wasm/00194036:wasm-function[466]:0x2a899
    at wasm://wasm/00194036:wasm-function[422]:0x29f0f
    at wasm://wasm/00194036:wasm-function[116]:0x1c1ee
    at wasm://wasm/00194036:wasm-function[268]:0x26321
    at wasm://wasm/00194036:wasm-function[135]:0x1e12f
    at wasm://wasm/00194036:wasm-function[259]:0x25dd8
    at wasm://wasm/00194036:wasm-function[231]:0x24bca at line 0, col 146576
{
  "exceptionDetails": {
    "columnNumber": 146576,
    "exception": {
      "className": "RuntimeError",
      "description": "RuntimeError: unreachable\n    at wasm://wasm/00194036:wasm-function[211]:0x23c91\n    at wasm://wasm/00194036:wasm-function[290]:0x26f3d\n    at wasm://wasm/00194036:wasm-function[439]:0x2a387\n    at wasm://wasm/00194036:wasm-function[466]:0x2a899\n    at wasm://wasm/00194036:wasm-function[422]:0x29f0f\n    at wasm://wasm/00194036:wasm-function[116]:0x1c1ee\n    at wasm://wasm/00194036:wasm-function[268]:0x26321\n    at wasm://wasm/00194036:wasm-function[135]:0x1e12f\n    at wasm://wasm/00194036:wasm-function[259]:0x25dd8\n    at wasm://wasm/00194036:wasm-function[231]:0x24bca",
      "preview": {
        "description": "RuntimeError: unreachable\n    at wasm://wasm/00194036:wasm-function[211]:0x23c91\n    at wasm://wasm/00194036:wasm-function[290]:0x26f3d\n    at wasm://wasm/00194036:wasm-function[439]:0x2a387\n    at wasm://wasm/00194036:wasm-function[466]:0x2a899\n    at wasm://wasm/00194036:wasm-function[422]:0x29f0f\n    at wasm://wasm/00194036:wasm-function[116]:0x1c1ee\n    at wasm://wasm/00194036:wasm-function[268]:0x26321\n    at wasm://wasm/00194036:wasm-function[135]:0x1e12f\n    at wasm://wasm/00194036:wasm-function[259]:0x25dd8\n    at wasm://wasm/00194036:wasm-function[231]:0x24bca",
        "entries": null,
        "overflow": false,
        "properties": [
          {
            "name": "stack",
            "subtype": null,
            "type": "string",
            "value": "RuntimeError: unreachable\n    at wasm://wasm/00194…t wasm://wasm/00194036:wasm-function[231]:0x24bca",
            "valuePreview": null
          },
          {
            "name": "message",
            "subtype": null,
            "type": "string",
            "value": "unreachable",
            "valuePreview": null
          }
        ],
        "subtype": "error",
        "type": "object"
      },
      "subtype": "error",
      "type": "object",
      "value": null
    },
    "lineNumber": 0,
    "text": "Uncaught (in promise)",
    "url": "wasm://wasm/00194036"
  },
  "timestamp": 1638831246179
}
A hanging Promise was canceled. This happens when the worker runtime is waiting for a Promise from JavaScript to resolve, but has detected that the Promise cannot possibly ever resolve because all code and events related to the Promise's request context have already finished.
Error: The script will never generate a response. at line 0, col -2
{
  "exceptionDetails": {
    "columnNumber": -2,
    "exception": {
      "className": "Error",
      "description": "Error: The script will never generate a response.",
      "preview": {
        "description": "Error: The script will never generate a response.",
        "entries": null,
        "overflow": false,
        "properties": [
          {
            "name": "stack",
            "subtype": null,
            "type": "string",
            "value": "Error: The script will never generate a response.",
            "valuePreview": null
          },
          {
            "name": "message",
            "subtype": null,
            "type": "string",
            "value": "The script will never generate a response.",
            "valuePreview": null
          }
        ],
        "subtype": "error",
        "type": "object"
      },
      "subtype": "error",
      "type": "object",
      "value": null
    },
    "lineNumber": 0,
    "text": "Uncaught (in response)",
    "url": "undefined"
  },
  "timestamp": 1638831246179
}
[2021-12-06 17:54:09] GET id_counter_cfw.em32.workers.dev/ HTTP/1.1 500 Internal Server Error
```