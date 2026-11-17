import { describe, it, expect, vi, beforeEach } from 'vitest'

vi.mock('gchessboard', () => ({}))

function buildAppDom(): void {
  document.body.innerHTML = `
    <select id="game-select"><option value="immortal">Immortal</option></select>
    <button id="share-btn"></button>
    <dialog id="share-dialog">
      <button id="close-share-dialog-btn"></button>
      <button id="tab-fen"></button>
      <button id="tab-pgn"></button>
      <div id="panel-fen"></div>
      <div id="panel-pgn"></div>
      <textarea id="share-fen-text"></textarea>
      <textarea id="share-pgn-text"></textarea>
      <button id="dialog-copy-fen-btn"></button>
      <button id="dialog-copy-pgn-btn"></button>
      <button id="dialog-download-pgn-btn"></button>
    </dialog>
    <div id="global-toast"><span id="toast-message"></span><button id="toast-close-btn"></button></div>
    <button id="start-pos-btn"></button>
    <button id="prev-move"></button>
    <button id="next-move"></button>
    <h1 id="game-title"></h1>
    <p id="game-players"></p>
    <dl id="game-headers"></dl>
    <div id="game-result-container"></div>
    <ol id="moves-list"></ol>
    <div id="move-announcer"></div>
    <g-chess-board id="board"></g-chess-board>
  `
  const dialog = document.getElementById('share-dialog') as any
  dialog.close = vi.fn(() => {
    dialog.open = false
  })
  dialog.showModal = vi.fn(() => {
    dialog.open = true
  })
}

describe('main.ts and app.ts', () => {
  beforeEach(() => {
    vi.resetModules()
    document.body.innerHTML = ''
    HTMLElement.prototype.scrollIntoView = vi.fn()
    window.history.replaceState({}, '', '/?game=evergreen')
    window.matchMedia = vi.fn().mockReturnValue({
      matches: false,
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
    }) as any
  })

  it('initializes app via main.ts and handles all UI/window events', async () => {
    buildAppDom()

    globalThis.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url === '/api/games') {
        return {
          json: async () => [
            { id: 'immortal', title: 'Immortal Game' },
            { id: 'evergreen', title: 'Evergreen Game' },
          ],
        }
      }
      if (url.startsWith('/api/headers')) {
        return {
          json: async () => ({
            Title: 'Evergreen Game',
            White: 'Anderssen',
            Black: 'Dufresne',
            Result: '1-0',
          }),
        }
      }
      if (url.startsWith('/api/moves')) {
        return { json: async () => ['e4', 'e5', 'Nf3'] }
      }
      if (url.startsWith('/api/fen')) {
        return { text: async () => 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1' }
      }
      return { text: async () => '[Event "Berlin"]' }
    }) as any

    const writeTextMock = vi.fn().mockResolvedValue(undefined)
    Object.defineProperty(navigator, 'clipboard', {
      value: { writeText: writeTextMock },
      configurable: true,
    })

    const createUrlSpy = vi.spyOn(URL, 'createObjectURL').mockReturnValue('blob:app')
    const revokeUrlSpy = vi.spyOn(URL, 'revokeObjectURL').mockImplementation(() => {})
    const clickSpy = vi.spyOn(HTMLAnchorElement.prototype, 'click').mockImplementation(() => {})

    await import('./main')

    const state = await import('./state')
    await vi.waitFor(() => {
      expect(state.currentGameId.value).toBe('evergreen')
      expect(state.moves.value).toEqual(['e4', 'e5', 'Nf3'])
    })

    const gameSelectEl = document.getElementById('game-select') as HTMLSelectElement
    expect(gameSelectEl.options.length).toBe(2)

    // Next / prev / start-pos buttons
    const prevBtn = document.getElementById('prev-move') as HTMLButtonElement
    const nextBtn = document.getElementById('next-move') as HTMLButtonElement
    const startBtn = document.getElementById('start-pos-btn') as HTMLButtonElement

    // prevBtn is aria-disabled="true" at move 0 -> click does nothing
    prevBtn.click()
    expect(state.currentMoveIndex.value).toBe(0)

    nextBtn.click()
    await vi.waitFor(() => expect(state.currentMoveIndex.value).toBe(1))

    prevBtn.click()
    await vi.waitFor(() => expect(state.currentMoveIndex.value).toBe(0))

    // Keyboard navigation on document.body (HTMLElement) + non-arrow key
    document.body.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowRight', bubbles: true }))
    await vi.waitFor(() => expect(state.currentMoveIndex.value).toBe(1))

    document.body.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowLeft', bubbles: true }))
    await vi.waitFor(() => expect(state.currentMoveIndex.value).toBe(0))

    document.body.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }))
    expect(state.currentMoveIndex.value).toBe(0)

    // Ignored keydown inside textarea
    const textarea = document.getElementById('share-fen-text')!
    textarea.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowRight', bubbles: true }))
    expect(state.currentMoveIndex.value).toBe(0)

    // Start pos button
    nextBtn.click()
    await vi.waitFor(() => expect(state.currentMoveIndex.value).toBe(1))
    startBtn.click()
    await vi.waitFor(() => expect(state.currentMoveIndex.value).toBe(0))

    // Disable nextBtn manually to hit aria-disabled branch
    nextBtn.setAttribute('aria-disabled', 'true')
    nextBtn.click()
    expect(state.currentMoveIndex.value).toBe(0)

    // Share dialog & tabs & copy & download & backdrop close
    document.getElementById('share-btn')!.click()
    await vi.waitFor(() => {
      expect((document.getElementById('share-dialog') as any).showModal).toHaveBeenCalled()
    })

    document.getElementById('tab-fen')!.click()
    expect(document.getElementById('panel-fen')!.hidden).toBe(false)
    document.getElementById('tab-pgn')!.click()
    expect(document.getElementById('panel-pgn')!.hidden).toBe(false)

    document.getElementById('dialog-copy-fen-btn')!.click()
    document.getElementById('dialog-copy-pgn-btn')!.click()
    await vi.waitFor(() => expect(writeTextMock).toHaveBeenCalledTimes(2))

    document.getElementById('dialog-download-pgn-btn')!.click()
    await vi.waitFor(() => expect(clickSpy).toHaveBeenCalled())

    document.getElementById('toast-close-btn')!.click()
    expect(document.getElementById('global-toast')!.classList.contains('visible')).toBe(false)

    // Backdrop click on dialog
    const dialog = document.getElementById('share-dialog') as any
    dialog.dispatchEvent(new MouseEvent('click'))
    expect(dialog.close).toHaveBeenCalled()

    document.getElementById('close-share-dialog-btn')!.click()
    expect(dialog.close).toHaveBeenCalledTimes(2)

    // Change game via select
    gameSelectEl.value = 'immortal'
    gameSelectEl.dispatchEvent(new Event('change'))
    await vi.waitFor(() => expect(state.currentGameId.value).toBe('immortal'))

    // Popstate event (different game, then same game)
    window.history.replaceState({}, '', '/?game=evergreen')
    window.dispatchEvent(new PopStateEvent('popstate'))
    await vi.waitFor(() => expect(state.currentGameId.value).toBe('evergreen'))
    window.dispatchEvent(new PopStateEvent('popstate'))
    expect(state.currentGameId.value).toBe('evergreen')

    createUrlSpy.mockRestore()
    revokeUrlSpy.mockRestore()
    clickSpy.mockRestore()
  })

  it('handles fetchGamesList failure and already-populated game-select', async () => {
    buildAppDom()
    const select = document.getElementById('game-select') as HTMLSelectElement
    const opt2 = document.createElement('option')
    opt2.value = 'evergreen'
    select.appendChild(opt2)

    window.history.replaceState({}, '', '/')
    globalThis.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url === '/api/games') throw new Error('network')
      if (url.startsWith('/api/headers')) return { json: async () => ({}) }
      if (url.startsWith('/api/moves')) return { json: async () => [] }
      return { text: async () => 'fen' }
    }) as any

    const { initApp } = await import('./app')
    await initApp()
    expect(select.options.length).toBe(2)

    // Now test fetchGamesList catch branch when options.length <= 1
    vi.resetModules()
    buildAppDom()
    await import('./app')
  })
})
