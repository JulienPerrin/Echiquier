import { describe, it, expect, vi, beforeEach } from 'vitest'

describe('ui/headers.ts', () => {
  beforeEach(() => {
    vi.resetModules()
    document.body.innerHTML = ''
  })

  it('covers null headers, missing DOM, date-only, fallback title, missing players, and unparseable Date tag', async () => {
    const emptyHeadersMod = await import('./headers')
    const stateMod1 = await import('../state')
    stateMod1.headers.value = null
    emptyHeadersMod.renderHeaders()
    stateMod1.headers.value = { Title: 'Only Title' }
    emptyHeadersMod.renderHeaders()

    vi.resetModules()
    document.body.innerHTML = `
      <h1 id="game-title"></h1>
      <p id="game-players"></p>
      <dl id="game-headers"></dl>
    `
    const { headers } = await import('../state')
    const { renderHeaders } = await import('./headers')

    headers.value = {
      White: 'Anderssen',
      Black: 'Kieseritzky',
      Event: 'London',
      Date: '1851.06.21',
    }
    renderHeaders()
    const titleEl = document.getElementById('game-title')!
    expect(titleEl.textContent).toContain('Anderssen vs Kieseritzky, London,')
    expect(titleEl.querySelector('time')?.getAttribute('datetime')).toBe('1851-06-21')
    expect(document.getElementById('game-players')?.hidden).toBe(true)

    headers.value = {
      White: 'Morphy',
      Site: 'Paris',
    }
    renderHeaders()
    expect(titleEl.textContent).toBe('Morphy, Paris')

    headers.value = { Date: '1851.06.21' }
    renderHeaders()
    expect(titleEl.querySelector('time')?.getAttribute('datetime')).toBe('1851-06-21')

    headers.value = { Date: 'circa 1851', Round: '1' }
    renderHeaders()
    expect(titleEl.textContent).toBe('circa 1851')
    expect(document.getElementById('game-headers')?.textContent).toContain('circa 1851')

    headers.value = { Title: 'Custom' }
    renderHeaders()
    expect(document.getElementById('game-players')?.textContent).toBe('White (?) vs Black (?)')
  })
})
