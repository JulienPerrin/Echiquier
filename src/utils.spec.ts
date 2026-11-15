import { describe, it, expect } from 'vitest'
import { formatPgnDate, computeGameTitle, isHeaderDefined } from './utils'

describe('Utility Functions', () => {
  it('formatPgnDate handles valid dates', () => {
    const result = formatPgnDate('2023/01/01')
    expect(result).not.toBeNull()
    if (result) {
      expect(result.datetime).toBe('2023-01-01')
      expect(result.formatted).toContain('2023')
    }
  })

  it('formatPgnDate handles invalid dates', () => {
    expect(formatPgnDate('invalid')).toBeNull()
  })

  it('computeGameTitle handles various headers', () => {
    const testHeaders = {
      Title: 'The Immortal Game',
      White: 'White',
      Black: 'Black',
      Site: 'Site',
      Date: '2023/01/01',
      Result: '1-0',
    }
    expect(computeGameTitle(testHeaders)).toBe('The Immortal Game')
  })

  it('isHeaderDefined correctly identifies valid headers', () => {
    expect(isHeaderDefined('Valid')).toBe(true)
    expect(isHeaderDefined('')).toBe(false)
    expect(isHeaderDefined('*')).toBe(false)
    expect(isHeaderDefined('??')).toBe(false)
  })

  it('computeGameTitle handles missing title with default', () => {
    const testHeaders = {
      White: 'White',
      Black: 'Black',
      Site: 'Site',
      Date: '2023/01/01',
    }
    const result = computeGameTitle(testHeaders)
    expect(result).toContain('White vs Black')
    expect(result).toContain('Site')
    expect(result).toContain('2023')
  })
})
