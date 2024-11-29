```bash
cd jsplugin && extism-js jsplugin.js -i jsplugin.d.ts -o jsplugin.wasm && cd ../
```

```bash
cd rustplugin && cargo build --target wasm32-unknown-unknown && cd ../
```

```bash
cargo run
```

sample output

```
trying 380kbs
trying 761kbs
trying 1142kbs
jsplugin/jsplugin.wasm failed with error error while executing at wasm backtrace:
    0: 0xc3ec8 - <unknown>!<wasm function 1207>
    1: 0x4e85e - <unknown>!<wasm function 439>
    2: 0x4f2be - <unknown>!<wasm function 447>
    3: 0x82d3d - <unknown>!<wasm function 769>
    4: 0x3f736 - <unknown>!<wasm function 426>
    5: 0x40633 - <unknown>!<wasm function 434>
    6: 0x42652 - <unknown>!<wasm function 434>
    7: 0x29876 - <unknown>!<wasm function 263>
    8: 0xd24b8 - <unknown>!<wasm function 1347>
    9: 0xd3e76 - <unknown>!<wasm function 1364>

Caused by:
    wasm trap: wasm `unreachable` instruction executed
trying 380kbs
trying 761kbs
trying 1142kbs
trying 1523kbs
trying 1904kbs
trying 2285kbs
trying 2666kbs
trying 3046kbs
trying 3427kbs
trying 3808kbs
rustplugin/target/wasm32-unknown-unknown/debug/rustplugin.wasm finished
```


