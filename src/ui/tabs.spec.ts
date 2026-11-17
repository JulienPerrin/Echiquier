import { describe, it, expect, vi, beforeEach } from 'vitest'

function buildTabsDom(): void {
  document.body.innerHTML = `
    <div id="app-tablist" role="tablist" aria-label="Application views">
      <button id="app-tab-view" type="button" role="tab" aria-selected="true" tabindex="0">View Games</button>
      <button id="app-tab-add" type="button" role="tab" aria-selected="false" tabindex="-1">Add Game</button>
    </div>
    <section id="panel-view-game" role="tabpanel" aria-labelledby="app-tab-view"></section>
    <section id="panel-add-game" role="tabpanel" aria-labelledby="app-tab-add" hidden>
      <g-chess-board id="add-game-board" coordinates="outside"></g-chess-board>
    </section>
  `
}

describe('ui/tabs.ts', () => {
  beforeEach(() => {
    vi.resetModules()
    document.body.innerHTML = ''
  })

  it('handles missing DOM elements gracefully', async () => {
    const { switchAppTab, initAppTabs } = await import('./tabs')
    switchAppTab('add')
    initAppTabs()
  })

  it('switches between view and add tabs and updates aria-selected, tabindex, and panel hidden state', async () => {
    buildTabsDom()
    const { switchAppTab, initAppTabs } = await import('./tabs')
    initAppTabs()

    const viewTab = document.getElementById('app-tab-view') as HTMLButtonElement
    const addTab = document.getElementById('app-tab-add') as HTMLButtonElement
    const viewPanel = document.getElementById('panel-view-game') as HTMLElement
    const addPanel = document.getElementById('panel-add-game') as HTMLElement

    expect(viewTab.getAttribute('aria-selected')).toBe('true')
    expect(addPanel.hidden).toBe(true)

    addTab.click()
    expect(viewTab.getAttribute('aria-selected')).toBe('false')
    expect(viewTab.tabIndex).toBe(-1)
    expect(addTab.getAttribute('aria-selected')).toBe('true')
    expect(addTab.tabIndex).toBe(0)
    expect(viewPanel.hidden).toBe(true)
    expect(addPanel.hidden).toBe(false)
    expect(addPanel.querySelector('#add-game-board')).not.toBeNull()

    viewTab.click()
    expect(viewTab.getAttribute('aria-selected')).toBe('true')
    expect(viewPanel.hidden).toBe(false)
    expect(addPanel.hidden).toBe(true)

    switchAppTab('add')
    expect(addTab.getAttribute('aria-selected')).toBe('true')
  })

  it('supports ArrowRight and ArrowLeft keyboard navigation on the tablist', async () => {
    buildTabsDom()
    const { initAppTabs } = await import('./tabs')
    initAppTabs()

    const tablist = document.getElementById('app-tablist')!
    const viewTab = document.getElementById('app-tab-view') as HTMLButtonElement
    const addTab = document.getElementById('app-tab-add') as HTMLButtonElement

    viewTab.focus()
    tablist.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowRight', bubbles: true }))
    expect(addTab.getAttribute('aria-selected')).toBe('true')
    expect(document.activeElement).toBe(addTab)

    tablist.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowLeft', bubbles: true }))
    expect(viewTab.getAttribute('aria-selected')).toBe('true')
    expect(document.activeElement).toBe(viewTab)

    tablist.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }))
    expect(viewTab.getAttribute('aria-selected')).toBe('true')
  })
})
