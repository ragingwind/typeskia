const gn_args = {
  // build configs
  is_official_build: true,
  is_debug: false,
  // target_cpu: "wasm",, "arm", "arm64", "armeabi-v7a", "arm64-v8a", "x64", "x86", "x86_64"
  // cc: "emcc",
  // cxx: "emcc",
  // ar: "emar",
  // skia features
  skia_enable_gpu: false,
  skia_enable_particles: true,
  skia_enable_skottie: false,
  skia_enable_skshaper: true,
  skia_enable_svg: false,
  skia_enable_skparagraph: true,
  skia_enable_sktext: true,
  skia_enable_tools: false,
  skia_enable_pdf: false,
  skia_enable_fontmgr_custom_directory: false,
  skia_enable_fontmgr_custom_embedded: true,
  skia_enable_fontmgr_custom_empty: false,
  skia_use_system_zlib: false,
  // text shaping, https://github.com/harfbuzz/harfbuzz
  skia_use_harfbuzz: false,
  skia_pdf_subset_harfbuzz: false,
  // unicode, https://icu.unicode.org/
  skia_use_icu: true,
  // skia_use_system_icu: false,
  // xml, https://libexpat.github.io/
  skia_use_expat: false,
  // font rendering, https://github.com/rillig/sfntly
  skia_use_sfntly: false,
  // font rendering, http://freetype.org/
  skia_use_freetype: true,
  skia_use_system_freetype2: false,
  skia_use_freetype_woff2: true,
  // graphic library, ANGEL(vulkan, metal, gl), codec configs
  skia_use_gl: false,
  skia_use_libjpeg_turbo_decode: false,
  skia_use_libjpeg_turbo_encode: false,
  skia_use_libwebp_decode: false,
  skia_use_libwebp_encode: false,
  skia_use_libpng_decode: true,
  skia_use_libpng_encode: false,
  skia_use_system_libpng: false,
  skia_use_libgifcodec: false,
  skia_use_libheif: false,
  // adobe dng sdk, reading and writing DNG files, https://android.googlesource.com/platform/external/dng_sdk/
  skia_use_dng_sdk: false,
  // preview image extractor, https://github.com/google/piex
  skia_use_piex: false,
  // wuffs programming language, https://github.com/google/wuffs
  skia_use_wuffs: true,
  // lua programming language, https://www.lua.org/
  skia_use_lua: false,
}

export function stringifyArgs() {
  return Object.entries(gn_args).map(([arg, v]) => `${arg}=${v}`).join(' ');
}
