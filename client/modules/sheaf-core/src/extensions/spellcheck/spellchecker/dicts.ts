/** A table of URLs describing where to retrieve a frequency dictionary for a locale. */
export interface Dictionary {
  /** A URL to the frequency dictionary for a locale. */
  dict: string
  /** An optional URL to a bigram frequency dictionary for a locale. */
  bigram?: string
}

/**
 * Table of functions, that when one is called, will return a absolute URL
 * to a spellcheck frequency dictionary. Key names should just be a
 * locale's language code, without any region attached.
 */
export const DICTIONARIES: Record<string, () => Promise<Dictionary>> = {
  // TODO: use @root when vite-tsconfig-paths gets support for suffixes
  // see: https://github.com/aleclarson/vite-tsconfig-paths/pull/29
  // looks like: await url(import("@root/locales/spellcheck/en.txt?url"))
  "en": async () => ({
    dict: await url(import("../../../../../../../locales/spellcheck/en.txt?url"))
  }),
  "de": async () => ({
    dict: await url(import("../../../../../../../locales/spellcheck/de.txt?url"))
  }),
  "es": async () => ({
    dict: await url(import("../../../../../../../locales/spellcheck/es.txt?url"))
  }),
  "fr": async () => ({
    dict: await url(import("../../../../../../../locales/spellcheck/fr.txt?url"))
  }),
  "he": async () => ({
    dict: await url(import("../../../../../../../locales/spellcheck/he.txt?url"))
  }),
  "it": async () => ({
    dict: await url(import("../../../../../../../locales/spellcheck/it.txt?url"))
  }),
  "ru": async () => ({
    dict: await url(import("../../../../../../../locales/spellcheck/ru.txt?url"))
  }),
  "zh": async () => ({
    dict: await url(import("../../../../../../../locales/spellcheck/zh.txt?url"))
  })
}

export default DICTIONARIES

/** Helper for turning a relative async. import into a absolute path. */
async function url(imp: Promise<any>) {
  return new URL((await imp).default, import.meta.url).toString()
}