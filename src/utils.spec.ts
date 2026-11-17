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

  it('isHeaderDefined handles undefined and punctuation-only values', () => {
    expect(isHeaderDefined(undefined)).toBe(false)
    expect(isHeaderDefined('   ')).toBe(false)
    expect(isHeaderDefined('---')).toBe(false)
  })

  it('formatPgnDate handles year-month, year-only, and invalid year', () => {
    const ym = formatPgnDate('2023.05.??')
    expect(ym?.datetime).toBe('2023-05')
    expect(ym?.formatted).toContain('2023')

    const y = formatPgnDate('1851.??.??')
    expect(y).toEqual({ datetime: '1851', formatted: '1851' })

    expect(formatPgnDate('0000.01.01')).toBeNull()
  })

  it('computeGameTitle handles partial headers and fallback', () => {
    expect(computeGameTitle({ White: 'Morphy' })).toBe('Morphy')
    expect(computeGameTitle({ Black: 'Anderssen' })).toBe('Anderssen')
    expect(computeGameTitle({ Event: 'London', Date: 'circa 1851' })).toBe('London, circa 1851')
    expect(computeGameTitle({})).toBe('The Immortal Game')
  })
})
