import * as wasm from '../wasm-lib/pkg/wasm_lib'
import Splitter from 'grapheme-splitter'

const splitter = new Splitter()

function render() {
  const display = document.getElementById('display')
  const data = wasm
    .fetch()
    .trimEnd()
    .split('\n')
    .map((x) => splitter.splitGraphemes(x))
  const lost = data.find((row) => row.includes('💣')) !== undefined
  if (display) {
    display.innerHTML = ''
    display.style.textAlign = 'center'
    for (let x = 0; x < data.length; x++) {
      const rowEl = document.createElement('div')
      for (let y = 0; y < (data[x]?.length || 0); y++) {
        const cellEl = document.createElement('a')
        cellEl.innerText = data[x]?.at(y) || ''
        if (cellEl.innerText === '⬛' && !lost) {
          cellEl.href = '#'
          cellEl.style.textDecoration = 'none'
          cellEl.addEventListener('click', (e) => {
            e.preventDefault()
            wasm.open(x, y)
            render()
          })
          cellEl.addEventListener('contextmenu', (e) => {
            e.preventDefault()
            wasm.flag(x, y)
            render()
          })
        }
        rowEl.appendChild(cellEl)
      }
      display.appendChild(rowEl)
    }
  }
}

document.getElementById('restart-btn')?.addEventListener('click', (e) => {
  e.preventDefault()
  wasm.start(16, 30, 70)
  render()
})

wasm.start(16, 30, 70)
render()
