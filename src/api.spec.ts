import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'

const invokeMock = vi.fn()
vi.mock('@tauri-apps/api/core', () => ({
  invoke: invokeMock,
}))

describe('api module', () => {
  const originalFetch = globalThis.fetch

  beforeEach(() => {
    vi.resetModules()
    invokeMock.mockReset()
    delete (window as any).__TAURI_INTERNALS__
  })

  afterEach(() => {
    globalThis.fetch = originalFetch
    delete (window as any).__TAURI_INTERNALS__
  })

  it('uses fetch in browser mode', async () => {
    const fetchMock = vi.fn().mockImplementation(async (url: string) => {
      if (url === '/api/games') {
        return { json: async () => [{ id: 'immortal', title: 'Immortal' }] }
      }
      if (url.startsWith('/api/headers')) {
        return { json: async () => ({ White: 'Anderssen' }) }
      }
      if (url.startsWith('/api/moves')) {
        return { json: async () => ['e4', 'e5'] }
      }
      if (url.startsWith('/api/fen')) {
        return { text: async () => '  startpos-fen \n' }
      }
      return { text: async () => '[Event "Test"]' }
    })
    globalThis.fetch = fetchMock as any

    const api = await import('./api')
    expect(api.isTauri).toBe(false)

    expect(await api.fetchGamesList()).toEqual([{ id: 'immortal', title: 'Immortal' }])
    expect(await api.fetchGameHeaders('g 1')).toEqual({ White: 'Anderssen' })
    expect(await api.fetchGameMoves('g 1')).toEqual(['e4', 'e5'])
    expect(await api.fetchGameFen(3, 'g 1')).toBe('startpos-fen')
    expect(await api.fetchGamePgn('g 1')).toBe('[Event "Test"]')

    expect(fetchMock).toHaveBeenCalledWith('/api/games')
    expect(fetchMock).toHaveBeenCalledWith('/api/headers?game=g%201')
    expect(fetchMock).toHaveBeenCalledWith('/api/moves?game=g%201')
    expect(fetchMock).toHaveBeenCalledWith('/api/fen?move=3&game=g%201')
    expect(fetchMock).toHaveBeenCalledWith('/api/pgn?game=g%201')
  })

  it('uses Tauri invoke when __TAURI_INTERNALS__ is present', async () => {
    ;(window as any).__TAURI_INTERNALS__ = {}
    invokeMock
      .mockResolvedValueOnce([{ id: 'immortal', title: 'Immortal' }])
      .mockResolvedValueOnce({ White: 'Morphy' })
      .mockResolvedValueOnce(['d4'])
      .mockResolvedValueOnce('fen-str')
      .mockResolvedValueOnce('pgn-str')

    const api = await import('./api')
    expect(api.isTauri).toBe(true)

    expect(await api.fetchGamesList()).toEqual([{ id: 'immortal', title: 'Immortal' }])
    expect(await api.fetchGameHeaders()).toEqual({ White: 'Morphy' })
    expect(await api.fetchGameMoves()).toEqual(['d4'])
    expect(await api.fetchGameFen(2)).toBe('fen-str')
    expect(await api.fetchGamePgn()).toBe('pgn-str')

    expect(invokeMock).toHaveBeenCalledWith('list_games')
    expect(invokeMock).toHaveBeenCalledWith('game_headers', { game: 'immortal' })
    expect(invokeMock).toHaveBeenCalledWith('game_moves', { game: 'immortal' })
    expect(invokeMock).toHaveBeenCalledWith('game_fen', { moveIndex: 2, game: 'immortal' })
    expect(invokeMock).toHaveBeenCalledWith('game_pgn', { game: 'immortal' })
  })
})
