import { describe, it, expect, beforeAll, vi } from 'vitest'

// Prepare DOM with all required elements before importing modules
beforeAll(() => {
  document.body.innerHTML = `
    <h1 id="game-title"></h1>
    <p id="game-players"></p>
    <dl id="game-headers"></dl>
    <div id="game-result-container"></div>
    <ol id="moves-list"></ol>
    <div id="global-toast" hidden><span id="toast-message"></span></div>
    <button id="tab-fen" class="share-tab"></button>
    <button id="tab-pgn" class="share-tab"></button>
    <g-chess-board id="board"></g-chess-board>
  `

  // Mock fetch for API functions
  global.fetch = vi.fn().mockResolvedValue({ json: async () => ([] as any) }) as any
})

// Helper to lazy import UI modules after DOM is ready
async function importUI() {
  const [headersMod, resultMod, movesMod, toastMod, boardMod, shareMod, stateMod] = await Promise.all([
    import('./ui/headers'),
    import('./ui/result'),
    import('./ui/moves'),
    import('./ui/toast'),
    import('./ui/board'),
    import('./ui/share'),
    import('./state'),
  ])
  return { ...headersMod, ...resultMod, ...movesMod, ...toastMod, ...boardMod, ...shareMod, ...stateMod }
}

let renderHeaders: typeof import('./ui/headers').renderHeaders
let renderGameResult: typeof import('./ui/result').renderGameResult
let renderMovesList: typeof import('./ui/moves').renderMovesList
let showToast: typeof import('./ui/toast').showToast
let hideToast: typeof import('./ui/toast').hideToast
let switchShareTab: typeof import('./ui/share').switchShareTab
let syncBoardMotionPreference: typeof import('./ui/board').syncBoardMotionPreference
let headers: typeof import('./state').headers
let moves: typeof import('./state').moves
let gameResult: typeof import('./state').gameResult

beforeAll(async () => {
  const ui = await importUI()
  renderHeaders = ui.renderHeaders
  renderGameResult = ui.renderGameResult
  renderMovesList = ui.renderMovesList
  showToast = ui.showToast
  hideToast = ui.hideToast
  switchShareTab = ui.switchShareTab
  syncBoardMotionPreference = ui.syncBoardMotionPreference
  headers = ui.headers
  moves = ui.moves
  gameResult = ui.gameResult
})

describe('UI components rendering', () => {
  it('renders headers with full header data', () => {
    const mockHeader = {
      Title: 'Sample Game',
      White: 'Alice',
      Black: 'Bob',
      Site: 'ChessClub',
      Date: '2023/01/01',
      Result: '1-0',
    }
    headers.value = mockHeader as any
    renderHeaders()
    const titleEl = document.getElementById('game-title') as HTMLHeadingElement
    const playersEl = document.getElementById('game-players') as HTMLParagraphElement
    const headersEl = document.getElementById('game-headers') as HTMLDListElement
    expect(titleEl.textContent?.trim()).toContain('Sample Game')
    expect(playersEl.textContent).toBe('Alice vs Bob')
    expect(headersEl.children.length).toBeGreaterThan(0)
  })

  it('renders game result badge', () => {
    gameResult.value = '1-0'
    renderGameResult()
    const container = document.getElementById('game-result-container') as HTMLDivElement
    expect(container.textContent?.trim()).toBe('1-0')
  })

  it('renders moves list', () => {
    moves.value = ['e4', 'e5', 'Nf3']
    renderMovesList()
    const ol = document.getElementById('moves-list') as HTMLOrderedListElement
    // There should be 2 rows because 3 moves => 1. e4 N/A, 2. e5 Nf3
    expect(ol.children.length).toBe(2)
    expect(ol.children[0].textContent).toContain('e4')
    expect(ol.children[1].textContent).toContain('Nf3')
  })

  it('shows and hides toast', () => {
    showToast('Hello world')
    const toast = document.getElementById('global-toast') as HTMLDivElement
    const messageEl = document.getElementById('toast-message') as HTMLSpanElement
    expect(toast.classList.contains('visible')).toBe(true)
    expect(messageEl.textContent).toBe('Hello world')
    hideToast()
    expect(toast.classList.contains('visible')).toBe(false)

  })

  it('switches share tab', () => {
    switchShareTab('fen')
    const fenBtn = document.getElementById('tab-fen') as HTMLElement
    const pgnBtn = document.getElementById('tab-pgn') as HTMLElement
    expect(fenBtn.classList.contains('active')).toBe(true)
    expect(pgnBtn.classList.contains('active')).toBe(false)
  })

  it('board respects reduced motion preference', () => {
    // Simulate prefers-reduced-motion query true
    const media = window.matchMedia('(prefers-reduced-motion: reduce)') as any
    media.matches = true
    // @ts-ignore
    window.matchMedia = () => media
    syncBoardMotionPreference()
    const board = document.querySelector('#board') as any
    expect(board.animationDuration).toBe(0)
  })
})
