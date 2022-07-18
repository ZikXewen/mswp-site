import * as wasm from '../wasm-lib/pkg/wasm_lib_bg.wasm'

let root = document.getElementById('root')
let elem = document.createElement('h3')
elem.innerText = wasm.add(1, 5).toString()
root?.appendChild(elem)
