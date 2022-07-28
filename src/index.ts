import * as wasm from 'mswp'
import Splitter from 'grapheme-splitter'

const splitter = new Splitter()

function render() {
  const display = document.getElementById('display')
  const data = wasm
    .fetch()
    .trimEnd()
    .split('\n')
    .map((x) => splitter.splitGraphemes(x))
  const ended = wasm.gameEnded()
  if (display) {
    display.innerHTML = ''
    display.style.textAlign = 'center'
    for (let x = 0; x < data.length; x++) {
      const rowEl = document.createElement('div')
      for (let y = 0; y < (data[x]?.length || 0); y++) {
        const cellEl = document.createElement('span')
        cellEl.style.width = '1.3rem'
        cellEl.style.display = 'inline-block'
        cellEl.innerText = data[x]?.at(y) || ''
        if (
          !ended &&
          (cellEl.innerText === '⬛' || cellEl.innerText === '🏳️')
        ) {
          cellEl.style.cursor = 'pointer'
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

function startGame() {
  wasm.start(16, 30, 70)
  render()
}

document.getElementById('restart-btn')?.addEventListener('click', (e) => {
  e.preventDefault()
  startGame()
})

startGame()
