import { describe, it, expect, vi, beforeEach } from 'vitest'

function buildMovesDom(): void {
  document.body.innerHTML = `
    <ol id="moves-list"></ol>
    <div id="move-announcer"></div>
    <button id="start-pos-btn"></button>
    <button id="prev-move"></button>
    <button id="next-move"></button>
    <dialog id="share-dialog">
      <textarea id="share-fen-text"></textarea>
    </dialog>
    <g-chess-board id="board"></g-chess-board>
  `
}

describe('ui/moves.ts', () => {
  beforeEach(() => {
    vi.resetModules()
    document.body.innerHTML = ''
    HTMLElement.prototype.scrollIntoView = vi.fn()
  })

  it('covers describeMove, renderMovesList, click handler, selectMove, and updateActiveSelection', async () => {
    const emptyMovesMod = await import('./moves')
    emptyMovesMod.renderMovesList()

    vi.resetModules()
    buildMovesDom()
    globalThis.fetch = vi.fn().mockResolvedValue({
      text: async () => 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1',
    }) as any

    const dialogEl = document.getElementById('share-dialog') as any
    dialogEl.open = true

    const stateMod = await import('../state')
    const movesMod = await import('./moves')

    stateMod.moves.value = ['e4', 'e5', 'Nf3']
    expect(movesMod.describeMove(0)).toBe('Initial position')
    expect(movesMod.describeMove(1)).toBe('Move 1 White: e4')
    expect(movesMod.describeMove(2)).toBe('Move 1 Black: e5')
    expect(movesMod.describeMove(99)).toBe('Move 50 White: ')

    movesMod.renderMovesList()

    await movesMod.selectMove(-1)
    await movesMod.selectMove(99)
    expect(stateMod.currentMoveIndex.value).toBe(0)

    const firstBtn = stateMod.moveButtons.get(1)!
    firstBtn.click()
    await vi.waitFor(() => {
      expect((document.getElementById('board') as any).fen).toContain('4P3')
    })
    expect(stateMod.currentMoveIndex.value).toBe(1)
    expect((document.getElementById('share-fen-text') as HTMLTextAreaElement).value).toContain('4P3')
    expect(document.getElementById('move-announcer')?.textContent).toBe('Move 1 White: e4')
    expect(document.getElementById('prev-move')?.getAttribute('aria-disabled')).toBe('false')
    expect(document.getElementById('next-move')?.getAttribute('aria-disabled')).toBe('false')

    window.matchMedia = vi.fn().mockReturnValue({ matches: true }) as any
    vi.resetModules()
    buildMovesDom()
    globalThis.fetch = vi.fn().mockResolvedValue({ text: async () => 'start-fen' }) as any
    const stateMod2 = await import('../state')
    const movesMod2 = await import('./moves')
    stateMod2.moves.value = ['e4']
    movesMod2.renderMovesList()
    await movesMod2.selectMove(1)
    await movesMod2.selectMove(0)
    expect(document.getElementById('start-pos-btn')?.classList.contains('active')).toBe(true)
    expect(document.getElementById('prev-move')?.getAttribute('aria-disabled')).toBe('true')

    vi.resetModules()
    document.body.innerHTML = `<ol id="moves-list"></ol>`
    globalThis.fetch = vi.fn().mockResolvedValue({ text: async () => 'start-fen' }) as any
    const stateMod3 = await import('../state')
    const movesMod3 = await import('./moves')
    stateMod3.moves.value = ['e4']
    movesMod3.renderMovesList()
    await movesMod3.selectMove(1)
    expect(stateMod3.currentMoveIndex.value).toBe(1)
  })
})
