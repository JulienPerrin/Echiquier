import { describe, it, expect, vi, beforeEach } from 'vitest'

function buildAddGameDom(): void {
  document.body.innerHTML = `
    <div id="global-toast"><span id="toast-message"></span></div>
    <section id="panel-add-game">
      <form id="add-game-form" aria-labelledby="add-game-heading">
        <h2 id="add-game-heading">Game Details</h2>
        <input id="add-header-title" name="title" type="text" />
        <input id="add-header-white" name="white" type="text" />
        <input id="add-header-black" name="black" type="text" />
        <input id="add-header-event" name="event" type="text" />
        <input id="add-header-site" name="site" type="text" />
        <input id="add-header-date" name="date" type="date" />
        <input id="add-header-round" name="round" type="text" />
        <select id="add-header-result" name="result">
          <option value="*">*</option>
          <option value="1-0">1-0</option>
          <option value="0-1">0-1</option>
          <option value="1/2-1/2">1/2-1/2</option>
        </select>
        <button id="add-save-game-btn" type="submit" hidden>Save Game</button>
      </form>
      <g-chess-board id="add-game-board" coordinates="outside"></g-chess-board>
      <button id="add-undo-btn" type="button"></button>
      <button id="add-import-pgn-btn" type="button"></button>
      <div id="add-import-pgn-box" hidden>
        <input id="add-import-pgn-file" type="file" accept=".pgn,text/plain" />
        <textarea id="add-import-pgn-text"></textarea>
        <button id="add-import-pgn-submit" type="button"></button>
        <button id="add-import-pgn-cancel" type="button"></button>
      </div>
      <ol id="add-moves-list"></ol>
    </section>
  `
}

describe('ui/add-game.ts', () => {
  beforeEach(() => {
    vi.resetModules()
    document.body.innerHTML = ''
    localStorage.clear()
  })

  it('handles missing DOM elements gracefully', async () => {
    const { initAddGame, renderAddGameMoves } = await import('./add-game')
    await initAddGame()
    renderAddGameMoves(['e4'])
  })

  it('initializes board in startup position and renders empty moves state', async () => {
    buildAddGameDom()
    const { initAddGame, START_FEN } = await import('./add-game')
    await initAddGame()

    const board = document.getElementById('add-game-board') as any
    expect(board.fen).toBe(START_FEN)

    const movesList = document.getElementById('add-moves-list')!
    expect(movesList.textContent).toContain('No moves played yet.')
  })

  it('renders played moves paired by turn on the right panel', async () => {
    buildAddGameDom()
    const { renderAddGameMoves } = await import('./add-game')
    renderAddGameMoves(['e4', 'e5', 'Nf3'])

    const movesList = document.getElementById('add-moves-list')!
    expect(movesList.children.length).toBe(2)
    expect(movesList.children[0].textContent).toContain('1.')
    expect(movesList.children[0].textContent).toContain('e4')
    expect(movesList.children[0].textContent).toContain('e5')
    expect(movesList.children[1].textContent).toContain('2.')
    expect(movesList.children[1].textContent).toContain('Nf3')
  })

  it('saves header form info and moves to localStorage on submit', async () => {
    buildAddGameDom()
    globalThis.fetch = vi.fn().mockResolvedValue({
      json: async () => ({
        fen: 'after-e4-fen',
        turn: 'black',
        san_moves: ['e4'],
        allowed_moves: {},
        result: null,
      }),
    }) as any
    const { initAddGame, getSavedGames } = await import('./add-game')
    const onSaved = vi.fn()
    await initAddGame(onSaved)

    const saveBtn = document.getElementById('add-save-game-btn') as HTMLButtonElement
    expect(saveBtn.hidden).toBe(true)

    const board = document.getElementById('add-game-board') as any
    board.dispatchEvent(new CustomEvent('movefinished', { detail: { from: 'e2', to: 'e4' } }))
    await vi.waitFor(() => expect(saveBtn.hidden).toBe(false))

    ;(document.getElementById('add-header-title') as HTMLInputElement).value = 'Club Match'
    ;(document.getElementById('add-header-white') as HTMLInputElement).value = 'Alice'
    ;(document.getElementById('add-header-black') as HTMLInputElement).value = 'Bob'
    ;(document.getElementById('add-header-event') as HTMLInputElement).value = 'Open'
    ;(document.getElementById('add-header-site') as HTMLInputElement).value = 'Paris'
    ;(document.getElementById('add-header-date') as HTMLInputElement).value = '2026-11-17'
    ;(document.getElementById('add-header-round') as HTMLInputElement).value = '3'
    ;(document.getElementById('add-header-result') as HTMLSelectElement).value = '1-0'

    const form = document.getElementById('add-game-form') as HTMLFormElement
    form.dispatchEvent(new Event('submit', { cancelable: true }))

    const saved = getSavedGames()
    expect(saved.length).toBe(1)
    expect(saved[0].headers).toEqual({
      Title: 'Club Match',
      White: 'Alice',
      Black: 'Bob',
      Event: 'Open',
      Site: 'Paris',
      Date: '2026.11.17',
      Round: '3',
      Result: '1-0',
    })
    expect(saved[0].moves).toEqual(['e4'])
    expect(onSaved).toHaveBeenCalledWith(saved[0])
    expect(document.getElementById('toast-message')?.textContent).toContain('Saved')
  })

  it('handles corrupted localStorage data gracefully', async () => {
    localStorage.setItem('echiquier.savedGames', '{bad-json')
    const { getSavedGames } = await import('./add-game')
    expect(getSavedGames()).toEqual([])
  })

  it('handles movestart, moveend, and movefinished using src-tauri position_state API', async () => {
    buildAddGameDom()
    globalThis.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url.includes('moves=e2e4')) {
        return {
          json: async () => ({
            fen: 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1',
            turn: 'black',
            san_moves: ['e4'],
            allowed_moves: { e7: ['e6', 'e5'] },
          }),
        }
      }
      return {
        json: async () => ({
          fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
          turn: 'white',
          san_moves: [],
          allowed_moves: { e2: ['e3', 'e4'] },
        }),
      }
    }) as any

    const { initAddGame } = await import('./add-game')
    await initAddGame()

    const board = document.getElementById('add-game-board') as any
    expect(board.turn).toBe('white')

    const setTargets = vi.fn()
    board.dispatchEvent(
      new CustomEvent('movestart', {
        detail: { from: 'e2', piece: { color: 'white', pieceType: 'pawn' }, setTargets },
      })
    )
    expect(setTargets).toHaveBeenCalledWith(['e3', 'e4'])

    const illegalEnd = new CustomEvent('moveend', {
      cancelable: true,
      detail: { from: 'e2', to: 'e5', piece: { color: 'white', pieceType: 'pawn' } },
    })
    board.dispatchEvent(illegalEnd)
    expect(illegalEnd.defaultPrevented).toBe(true)

    const legalEnd = new CustomEvent('moveend', {
      cancelable: true,
      detail: { from: 'e2', to: 'e4', piece: { color: 'white', pieceType: 'pawn' } },
    })
    board.dispatchEvent(legalEnd)
    expect(legalEnd.defaultPrevented).toBe(false)

    board.dispatchEvent(
      new CustomEvent('movefinished', {
        detail: { from: 'e2', to: 'e4', piece: { color: 'white', pieceType: 'pawn' } },
      })
    )

    await vi.waitFor(() => {
      expect(board.turn).toBe('black')
    })
    expect(board.fen).toContain('4P3')
    expect(document.getElementById('add-moves-list')?.textContent).toContain('e4')
  })

  it('disables result on checkmate/stalemate, still saves it, and unlocks when going back', async () => {
    buildAddGameDom()
    globalThis.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url.includes('moves=d8h4')) {
        return {
          json: async () => ({
            fen: 'mate-fen',
            turn: 'white',
            san_moves: ['Qh4#'],
            allowed_moves: {},
            result: '0-1',
          }),
        }
      }
      return {
        json: async () => ({
          fen: 'start-fen',
          turn: 'white',
          san_moves: [],
          allowed_moves: { d8: ['h4'] },
          result: null,
        }),
      }
    }) as any

    const { initAddGame, getSavedGames } = await import('./add-game')
    const onSaved = vi.fn()
    await initAddGame(onSaved)

    const resultSelect = document.getElementById('add-header-result') as HTMLSelectElement
    expect(resultSelect.value).toBe('*')
    expect(resultSelect.disabled).toBe(false)

    const board = document.getElementById('add-game-board') as any
    board.dispatchEvent(
      new CustomEvent('movefinished', {
        detail: { from: 'd8', to: 'h4', piece: { color: 'black', pieceType: 'queen' } },
      })
    )

    await vi.waitFor(() => {
      expect(resultSelect.value).toBe('0-1')
    })
    expect(resultSelect.disabled).toBe(true)

    const form = document.getElementById('add-game-form') as HTMLFormElement
    form.dispatchEvent(new Event('submit', { cancelable: true }))
    expect(getSavedGames()[0].headers.Result).toBe('0-1')
  })

  it('supports undoing moves via #add-undo-btn and resets auto-set mate result', async () => {
    buildAddGameDom()
    globalThis.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url.includes('moves=f2f3%2Cd8h4')) {
        return {
          json: async () => ({
            fen: 'mate-fen',
            turn: 'white',
            san_moves: ['f3', 'Qh4#'],
            allowed_moves: {},
            result: '0-1',
          }),
        }
      }
      if (url.includes('moves=f2f3')) {
        return {
          json: async () => ({
            fen: 'after-f3-fen',
            turn: 'black',
            san_moves: ['f3'],
            allowed_moves: { d8: ['h4'] },
            result: null,
          }),
        }
      }
      return {
        json: async () => ({
          fen: 'start-fen',
          turn: 'white',
          san_moves: [],
          allowed_moves: { f2: ['f3'] },
          result: null,
        }),
      }
    }) as any

    const { initAddGame } = await import('./add-game')
    await initAddGame()

    const undoBtn = document.getElementById('add-undo-btn') as HTMLButtonElement
    const resultSelect = document.getElementById('add-header-result') as HTMLSelectElement
    const board = document.getElementById('add-game-board') as any

    expect(undoBtn.disabled).toBe(true)

    board.dispatchEvent(new CustomEvent('movefinished', { detail: { from: 'f2', to: 'f3' } }))
    await vi.waitFor(() => expect(board.turn).toBe('black'))
    expect(undoBtn.disabled).toBe(false)

    board.dispatchEvent(new CustomEvent('movefinished', { detail: { from: 'd8', to: 'h4' } }))
    await vi.waitFor(() => expect(resultSelect.value).toBe('0-1'))
    expect(resultSelect.disabled).toBe(true)

    // Undo checkmate move -> result resets to '*', unlocks, and board returns to black's turn
    undoBtn.click()
    await vi.waitFor(() => expect(board.turn).toBe('black'))
    expect(resultSelect.value).toBe('*')
    expect(resultSelect.disabled).toBe(false)

    // Result can still be changed manually afterwards
    resultSelect.value = '1-0'
    expect(resultSelect.value).toBe('1-0')

    // Undo remaining move -> back to start, undo disabled
    undoBtn.click()
    await vi.waitFor(() => expect(board.turn).toBe('white'))
    expect(undoBtn.disabled).toBe(true)
  })

  it('goes back to a clicked move in #add-moves-list', async () => {
    buildAddGameDom()
    globalThis.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url.includes('moves=e2e4%2Ce7e5%2Cg1f3')) {
        return {
          json: async () => ({
            fen: 'after-nf3-fen',
            turn: 'black',
            san_moves: ['e4', 'e5', 'Nf3'],
            allowed_moves: {},
            result: null,
          }),
        }
      }
      if (url.includes('moves=e2e4%2Ce7e5')) {
        return {
          json: async () => ({
            fen: 'after-e5-fen',
            turn: 'white',
            san_moves: ['e4', 'e5'],
            allowed_moves: { g1: ['f3'] },
            result: null,
          }),
        }
      }
      if (url.includes('moves=e2e4')) {
        return {
          json: async () => ({
            fen: 'after-e4-fen',
            turn: 'black',
            san_moves: ['e4'],
            allowed_moves: { e7: ['e5'] },
            result: null,
          }),
        }
      }
      return {
        json: async () => ({
          fen: 'start-fen',
          turn: 'white',
          san_moves: [],
          allowed_moves: { e2: ['e4'] },
          result: null,
        }),
      }
    }) as any

    const { initAddGame } = await import('./add-game')
    await initAddGame()

    const board = document.getElementById('add-game-board') as any
    board.dispatchEvent(new CustomEvent('movefinished', { detail: { from: 'e2', to: 'e4' } }))
    await vi.waitFor(() => expect(board.turn).toBe('black'))

    board.dispatchEvent(new CustomEvent('movefinished', { detail: { from: 'e7', to: 'e5' } }))
    await vi.waitFor(() => expect(board.turn).toBe('white'))

    board.dispatchEvent(new CustomEvent('movefinished', { detail: { from: 'g1', to: 'f3' } }))
    await vi.waitFor(() => expect(board.fen).toBe('after-nf3-fen'))

    const buttons = document.querySelectorAll<HTMLButtonElement>('#add-moves-list button.move-btn')
    expect(buttons.length).toBe(3)

    // Click first move (e4) -> goes back to e4
    buttons[0].click()
    await vi.waitFor(() => expect(board.fen).toBe('after-e4-fen'))
    expect(document.querySelectorAll('#add-moves-list button.move-btn').length).toBe(1)
  })

  it('imports a PGN only when board is on initial position and populates headers, board, and moves', async () => {
    buildAddGameDom()
    globalThis.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url === '/api/import_pgn') {
        return {
          ok: true,
          json: async () => ({
            headers: {
              Title: 'Imported Match',
              White: 'Alice',
              Black: 'Bob',
              Date: '2026.11.17',
              Result: '0-1',
            },
            coord_moves: ['f2f3', 'e7e5', 'g2g4', 'd8h4'],
            state: {
              fen: 'mate-fen',
              turn: 'white',
              san_moves: ['f3', 'e5', 'g4', '♕h4#'],
              allowed_moves: {},
              result: '0-1',
            },
          }),
        }
      }
      return {
        json: async () => ({
          fen: 'start-fen',
          turn: 'white',
          san_moves: [],
          allowed_moves: { e2: ['e4'] },
          result: null,
        }),
      }
    }) as any

    const { initAddGame } = await import('./add-game')
    await initAddGame()

    const importBtn = document.getElementById('add-import-pgn-btn') as HTMLButtonElement
    const importBox = document.getElementById('add-import-pgn-box') as HTMLElement
    const importText = document.getElementById('add-import-pgn-text') as HTMLTextAreaElement
    const importSubmit = document.getElementById('add-import-pgn-submit') as HTMLButtonElement
    const importCancel = document.getElementById('add-import-pgn-cancel') as HTMLButtonElement
    const board = document.getElementById('add-game-board') as any
    const resultSelect = document.getElementById('add-header-result') as HTMLSelectElement

    expect(importBtn.hidden).toBe(false)
    importBtn.click()
    expect(importBox.hidden).toBe(false)
    expect(importBtn.hidden).toBe(true)

    importCancel.click()
    expect(importBox.hidden).toBe(true)
    expect(importBtn.hidden).toBe(false)

    importBtn.click()
    expect(importBtn.hidden).toBe(true)

    importText.value = '[Title "Imported Match"]\n\n1. f3 e5 2. g4 Qh4# 0-1'
    importSubmit.click()

    await vi.waitFor(() => expect(board.fen).toBe('mate-fen'))
    expect((document.getElementById('add-header-title') as HTMLInputElement).value).toBe('Imported Match')
    expect((document.getElementById('add-header-date') as HTMLInputElement).value).toBe('2026-11-17')
    expect(resultSelect.value).toBe('0-1')
    expect(resultSelect.disabled).toBe(true)
    expect(importBox.hidden).toBe(true)
    expect(importBtn.hidden).toBe(true)
    expect(document.querySelectorAll('#add-moves-list button.move-btn').length).toBe(4)
  })

  it('assigns a default "Game saved ..." title when saving with no headers', async () => {
    buildAddGameDom()
    globalThis.fetch = vi.fn().mockResolvedValue({
      json: async () => ({
        fen: 'after-e4-fen',
        turn: 'black',
        san_moves: ['e4'],
        allowed_moves: {},
        result: null,
      }),
    }) as any
    const { initAddGame, getSavedGames } = await import('./add-game')
    await initAddGame()

    const board = document.getElementById('add-game-board') as any
    board.dispatchEvent(new CustomEvent('movefinished', { detail: { from: 'e2', to: 'e4' } }))
    await vi.waitFor(() => expect(board.turn).toBe('black'))

    const form = document.getElementById('add-game-form') as HTMLFormElement
    form.dispatchEvent(new Event('submit', { cancelable: true }))

    const saved = getSavedGames()
    expect(saved[0].headers.Title).toMatch(/^Game saved \d{4}-\d{2}-\d{2} \d{2}:\d{2}$/)
  })

  it('reuses computeGameTitle for document.title when player names are entered', async () => {
    buildAddGameDom()
    globalThis.fetch = vi.fn().mockResolvedValue({
      json: async () => ({
        fen: 'after-e4-fen',
        turn: 'black',
        san_moves: ['e4'],
        allowed_moves: {},
        result: null,
      }),
    }) as any
    const { initAddGame, getSavedGames } = await import('./add-game')
    await initAddGame()

    const form = document.getElementById('add-game-form') as HTMLFormElement
    ;(document.getElementById('add-header-white') as HTMLInputElement).value = 'Alice'
    ;(document.getElementById('add-header-black') as HTMLInputElement).value = 'Bob'
    form.dispatchEvent(new Event('input', { bubbles: true }))

    expect(document.title).toBe('Alice vs Bob | Echiquier')

    const board = document.getElementById('add-game-board') as any
    board.dispatchEvent(new CustomEvent('movefinished', { detail: { from: 'e2', to: 'e4' } }))
    await vi.waitFor(() => expect(board.turn).toBe('black'))

    form.dispatchEvent(new Event('submit', { cancelable: true }))
    const saved = getSavedGames()
    expect(saved[0].headers.Title).toBeUndefined()
  })

  it('imports a PGN from an uploaded file', async () => {
    buildAddGameDom()
    globalThis.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url === '/api/import_pgn') {
        return {
          ok: true,
          json: async () => ({
            headers: { Title: 'Uploaded Game' },
            coord_moves: ['e2e4'],
            state: {
              fen: 'after-e4-fen',
              turn: 'black',
              san_moves: ['e4'],
              allowed_moves: {},
              result: null,
            },
          }),
        }
      }
      return {
        json: async () => ({
          fen: 'start-fen',
          turn: 'white',
          san_moves: [],
          allowed_moves: { e2: ['e4'] },
          result: null,
        }),
      }
    }) as any

    const { initAddGame } = await import('./add-game')
    await initAddGame()

    const fileInput = document.getElementById('add-import-pgn-file') as HTMLInputElement
    const mockFile = { text: async () => '[Title "Uploaded Game"]\n\n1. e4' }
    Object.defineProperty(fileInput, 'files', { value: [mockFile], configurable: true })
    fileInput.dispatchEvent(new Event('change'))

    const board = document.getElementById('add-game-board') as any
    await vi.waitFor(() => expect(board.fen).toBe('after-e4-fen'))
    expect((document.getElementById('add-header-title') as HTMLInputElement).value).toBe('Uploaded Game')
  })
})
