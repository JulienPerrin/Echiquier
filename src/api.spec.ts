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
    localStorage.clear()
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
      if (url.startsWith('/api/position_state')) {
        return {
          json: async () => ({
            fen: 'fen-after-e4',
            turn: 'black',
            san_moves: ['e4'],
            allowed_moves: { e7: ['e6', 'e5'] },
          }),
        }
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
    expect(await api.fetchPositionState(['e2e4'])).toEqual({
      fen: 'fen-after-e4',
      turn: 'black',
      san_moves: ['e4'],
      allowed_moves: { e7: ['e6', 'e5'] },
    })

    expect(fetchMock).toHaveBeenCalledWith('/api/games')
    expect(fetchMock).toHaveBeenCalledWith('/api/headers?game=g%201')
    expect(fetchMock).toHaveBeenCalledWith('/api/moves?game=g%201')
    expect(fetchMock).toHaveBeenCalledWith('/api/fen?move=3&game=g%201')
    expect(fetchMock).toHaveBeenCalledWith('/api/pgn?game=g%201')
    expect(fetchMock).toHaveBeenCalledWith('/api/position_state?moves=e2e4')
  })

  it('uses Tauri invoke when __TAURI_INTERNALS__ is present', async () => {
    ;(window as any).__TAURI_INTERNALS__ = {}
    invokeMock
      .mockResolvedValueOnce([{ id: 'immortal', title: 'Immortal' }])
      .mockResolvedValueOnce({ White: 'Morphy' })
      .mockResolvedValueOnce(['d4'])
      .mockResolvedValueOnce('fen-str')
      .mockResolvedValueOnce('pgn-str')
      .mockResolvedValueOnce({
        fen: 'fen-str',
        turn: 'white',
        san_moves: [],
        allowed_moves: { e2: ['e3', 'e4'] },
      })

    const api = await import('./api')
    expect(api.isTauri).toBe(true)

    expect(await api.fetchGamesList()).toEqual([{ id: 'immortal', title: 'Immortal' }])
    expect(await api.fetchGameHeaders()).toEqual({ White: 'Morphy' })
    expect(await api.fetchGameMoves()).toEqual(['d4'])
    expect(await api.fetchGameFen(2)).toBe('fen-str')
    expect(await api.fetchGamePgn()).toBe('pgn-str')
    expect(await api.fetchPositionState([])).toEqual({
      fen: 'fen-str',
      turn: 'white',
      san_moves: [],
      allowed_moves: { e2: ['e3', 'e4'] },
    })

    expect(invokeMock).toHaveBeenCalledWith('list_games')
    expect(invokeMock).toHaveBeenCalledWith('game_headers', { game: 'immortal' })
    expect(invokeMock).toHaveBeenCalledWith('game_moves', { game: 'immortal' })
    expect(invokeMock).toHaveBeenCalledWith('game_fen', { moveIndex: 2, game: 'immortal' })
    expect(invokeMock).toHaveBeenCalledWith('game_pgn', { game: 'immortal' })
    expect(invokeMock).toHaveBeenCalledWith('position_state', { moves: [] })
  })

  it('reads saved games from localStorage for list, headers, moves, fen, and pgn', async () => {
    localStorage.setItem(
      'echiquier.savedGames',
      JSON.stringify([
        {
          id: 'local-1',
          headers: { Title: 'My Club Game', White: 'Alice', Black: 'Bob', Result: '1-0' },
          moves: ['e4', 'e5'],
          coordMoves: ['e2e4', 'e7e5'],
        },
      ])
    )

    globalThis.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url === '/api/games') {
        return { json: async () => [{ id: 'immortal', title: 'Immortal' }] }
      }
      if (url.startsWith('/api/position_state')) {
        return {
          json: async () => ({
            fen: 'fen-after-e4',
            turn: 'black',
            san_moves: ['e4'],
            allowed_moves: {},
          }),
        }
      }
      return { json: async () => ({}) }
    }) as any

    const api = await import('./api')
    expect(await api.fetchGamesList()).toEqual([
      { id: 'immortal', title: 'Immortal' },
      { id: 'local-1', title: 'My Club Game' },
    ])
    expect(await api.fetchGameHeaders('local-1')).toEqual({
      Title: 'My Club Game',
      White: 'Alice',
      Black: 'Bob',
      Result: '1-0',
    })
    expect(await api.fetchGameMoves('local-1')).toEqual(['e4', 'e5'])
    expect(await api.fetchGameFen(1, 'local-1')).toBe('fen-after-e4')
    const pgn = await api.fetchGamePgn('local-1')
    expect(pgn).toContain('[Title "My Club Game"]')
    expect(pgn).toContain('1. e4 e5 1-0')
  })

  it('imports PGN in browser and Tauri modes', async () => {
    const importedPayload = {
      headers: { White: 'Alice', Black: 'Bob' },
      coord_moves: ['e2e4'],
      state: { fen: 'after-e4', turn: 'black', san_moves: ['e4'], allowed_moves: {}, result: null },
    }
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => importedPayload,
    }) as any

    const apiBrowser = await import('./api')
    expect(await apiBrowser.importPgn('1. e4')).toEqual(importedPayload)
    expect(globalThis.fetch).toHaveBeenCalledWith('/api/import_pgn', { method: 'POST', body: '1. e4' })

    vi.resetModules()
    ;(window as any).__TAURI_INTERNALS__ = {}
    invokeMock.mockResolvedValueOnce(importedPayload)
    const apiTauri = await import('./api')
    expect(await apiTauri.importPgn('1. e4')).toEqual(importedPayload)
    expect(invokeMock).toHaveBeenCalledWith('import_pgn', { pgn: '1. e4' })
  })
})
