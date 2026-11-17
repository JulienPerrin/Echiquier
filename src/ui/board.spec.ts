import { describe, it, expect, vi, beforeEach } from 'vitest'

describe('ui/board.ts', () => {
  beforeEach(() => {
    vi.resetModules()
    document.body.innerHTML = ''
  })

  it('handles missing board and non-reduced motion', async () => {
    const boardMod = await import('./board')
    boardMod.syncBoardMotionPreference()

    document.body.innerHTML = `<g-chess-board id="board"></g-chess-board>`
    window.matchMedia = vi.fn().mockReturnValue({ matches: false }) as any
    boardMod.syncBoardMotionPreference()
    expect((document.getElementById('board') as any).animationDuration).toBe(200)
  })
})
