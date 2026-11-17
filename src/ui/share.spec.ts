import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'

const invokeMock = vi.fn()
const tauriWriteTextMock = vi.fn()

vi.mock('@tauri-apps/api/core', () => ({
  invoke: invokeMock,
}))

vi.mock('@tauri-apps/plugin-clipboard-manager', () => ({
  writeText: tauriWriteTextMock,
}))

function buildShareDom(): void {
  document.body.innerHTML = `
    <div id="global-toast"><span id="toast-message"></span></div>
    <dialog id="share-dialog">
      <button id="tab-fen"></button>
      <button id="tab-pgn"></button>
      <div id="panel-fen"></div>
      <div id="panel-pgn"></div>
      <textarea id="share-fen-text"></textarea>
      <textarea id="share-pgn-text"></textarea>
    </dialog>
  `
}

describe('ui/share.ts', () => {
  beforeEach(() => {
    vi.resetModules()
    invokeMock.mockReset()
    tauriWriteTextMock.mockReset()
    delete (window as any).__TAURI_INTERNALS__
    document.body.innerHTML = ''
  })

  afterEach(() => {
    delete (window as any).__TAURI_INTERNALS__
  })

  it('covers copyToClipboard, switchShareTab, openShareDialog, and downloadPgn in browser and Tauri modes', async () => {
    const shareEmpty = await import('./share')
    shareEmpty.switchShareTab('pgn')
    await shareEmpty.openShareDialog()
    await shareEmpty.downloadPgn()

    Object.defineProperty(navigator, 'clipboard', { value: undefined, configurable: true })
    expect(await shareEmpty.copyToClipboard('abc', 'FEN')).toBe(false)

    vi.resetModules()
    buildShareDom()
    globalThis.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url.startsWith('/api/fen')) return { text: async () => 'fen-data' }
      return { text: async () => 'pgn-data' }
    }) as any

    const writeTextMock = vi.fn().mockResolvedValue(undefined)
    Object.defineProperty(navigator, 'clipboard', {
      value: { writeText: writeTextMock },
      configurable: true,
    })

    const dialogEl = document.getElementById('share-dialog') as any
    dialogEl.open = false
    dialogEl.showModal = vi.fn()

    const shareBrowser = await import('./share')
    expect(await shareBrowser.copyToClipboard('fen-data', 'FEN')).toBe(true)
    expect(writeTextMock).toHaveBeenCalledWith('fen-data')

    writeTextMock.mockRejectedValueOnce(new Error('denied'))
    expect(await shareBrowser.copyToClipboard('fen-data', 'FEN')).toBe(false)

    await shareBrowser.openShareDialog('fen')
    expect((document.getElementById('share-fen-text') as HTMLTextAreaElement).value).toBe('fen-data')
    expect((document.getElementById('share-pgn-text') as HTMLTextAreaElement).value).toBe('pgn-data')
    expect(document.getElementById('panel-fen')?.hidden).toBe(false)
    expect(document.getElementById('panel-pgn')?.hidden).toBe(true)
    expect(dialogEl.showModal).toHaveBeenCalled()

    const createUrlSpy = vi.spyOn(URL, 'createObjectURL').mockReturnValue('blob:mock')
    const revokeUrlSpy = vi.spyOn(URL, 'revokeObjectURL').mockImplementation(() => {})
    const clickSpy = vi.spyOn(HTMLAnchorElement.prototype, 'click').mockImplementation(() => {})

    await shareBrowser.downloadPgn()
    expect(createUrlSpy).toHaveBeenCalled()
    expect(clickSpy).toHaveBeenCalled()
    expect(revokeUrlSpy).toHaveBeenCalledWith('blob:mock')
    expect(document.getElementById('toast-message')?.textContent).toContain('Downloaded')

    createUrlSpy.mockRestore()
    revokeUrlSpy.mockRestore()
    clickSpy.mockRestore()

    dialogEl.open = true
    document.getElementById('share-fen-text')?.remove()
    document.getElementById('share-pgn-text')?.remove()
    await shareBrowser.openShareDialog()
    expect(dialogEl.showModal).toHaveBeenCalledTimes(1)

    vi.resetModules()
    buildShareDom()
    ;(window as any).__TAURI_INTERNALS__ = {}
    const shareTauri = await import('./share')

    tauriWriteTextMock.mockResolvedValueOnce(undefined)
    expect(await shareTauri.copyToClipboard('pgn-data', 'PGN')).toBe(true)
    expect(tauriWriteTextMock).toHaveBeenCalledWith('pgn-data')

    ;(document.getElementById('share-pgn-text') as HTMLTextAreaElement).value = 'pgn-data'
    invokeMock.mockResolvedValueOnce('/tmp/immortal.pgn')
    await shareTauri.downloadPgn()
    expect(invokeMock).toHaveBeenCalledWith('download_pgn', { game: 'immortal', pgn: 'pgn-data' })
    expect(document.getElementById('toast-message')?.textContent).toBe('✓ Saved to /tmp/immortal.pgn')

    invokeMock.mockRejectedValueOnce(new Error('disk full'))
    await shareTauri.downloadPgn()
    expect(document.getElementById('toast-message')?.textContent).toContain('Failed to save PGN')
  })
})
