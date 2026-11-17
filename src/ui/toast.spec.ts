import { describe, it, expect, vi, beforeEach } from 'vitest'

describe('ui/toast.ts', () => {
  beforeEach(() => {
    vi.resetModules()
    document.body.innerHTML = ''
  })

  it('handles missing elements, fallback without #toast-message, and popover methods', async () => {
    const emptyToast = await import('./toast')
    emptyToast.showToast('ignored')
    emptyToast.hideToast()

    vi.resetModules()
    document.body.innerHTML = `<div id="global-toast"></div>`
    const toastEl = document.getElementById('global-toast') as any
    toastEl.showPopover = vi.fn(() => {
      throw new Error('popover fail')
    })
    toastEl.hidePopover = vi.fn(() => {
      throw new Error('popover fail')
    })

    const toastMod = await import('./toast')
    toastMod.showToast('Fallback text')
    expect(toastEl.textContent).toBe('Fallback text')
    expect(toastEl.classList.contains('visible')).toBe(true)
    expect(toastEl.showPopover).toHaveBeenCalled()

    toastMod.hideToast()
    expect(toastEl.classList.contains('visible')).toBe(false)
    expect(toastEl.hidePopover).toHaveBeenCalled()
  })
})
