import { describe, it, expect, vi, beforeEach } from 'vitest'

describe('ui/result.ts', () => {
  beforeEach(() => {
    vi.resetModules()
    document.body.innerHTML = ''
  })

  it('handles missing container and undefined/empty result', async () => {
    const missingResult = await import('./result')
    missingResult.renderGameResult()

    vi.resetModules()
    document.body.innerHTML = `<div id="game-result-container"></div>`
    const { gameResult } = await import('../state')
    const { renderGameResult } = await import('./result')
    gameResult.value = '*'
    renderGameResult()
    expect(document.getElementById('game-result-container')?.innerHTML).toBe('')
  })
})
