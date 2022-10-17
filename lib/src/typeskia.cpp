#include <iostream>

// #include <include/gpu/GrDirectContext.h>
// #include <include/gpu/gl/GrGLInterface.h>
// #include <include/core/SkAlphaType.h>
#include <include/core/SkCanvas.h>
#include <include/core/SkColorSpace.h>
#include <include/core/SkColorType.h>
#include <include/core/SkData.h>
#include <include/core/SkFont.h>
#include <include/core/SkFontMgr.h>
#include <include/core/SkGraphics.h>
#include <include/core/SkImage.h>
#include <include/core/SkPaint.h>
#include <include/core/SkPath.h>
#include <include/core/SkStream.h>
#include <include/core/SkSurface.h>
#include <unistd.h>

#include <cassert>

using namespace std;

#include "typeskia.h"

#define DUMP_PTR(ptr) printf("ptr: %p\n", ptr)

SkFont loadFont(const string& font_path) {
  SkScalar textScale = 24;

  string font_file = font_path + "/gill-sans-italic.otf";
  cout << "font_file " << font_file << endl;

  sk_sp<SkFontMgr> mgr(SkFontMgr::RefDefault());
  sk_sp<SkTypeface> typeface = mgr->makeFromFile(font_file.c_str());

  DUMP_PTR(typeface.get());
  SkString familyName;
  typeface->getFamilyName(&familyName);

  printf("familyName: %s\n", familyName.c_str());

  // SkFont(sk_sp<SkTypeface> typeface, SkScalar size);
  SkFont font(typeface, textScale);
  return font;
}

void draw(int width, int height, const char *text, const char *font_path, const char *_path) {
  // SkImageInfo info = SkImageInfo::MakeN32Premul(width, height);
  // sk_sp<SkSurface> surface(SkSurface::MakeRaster(info));

  auto surface = SkSurface::MakeRasterN32Premul(width, height);
  SkCanvas *canvas = surface->getCanvas();
  SkPaint paint;
  SkFont font = loadFont(font_path);

  // font.setSize(SkIntToScalar(20));
  font.setEdging(SkFont::Edging::kAntiAlias);
  canvas->clear(SK_ColorRED);
  canvas->drawSimpleText("Hello World!", 12, SkTextEncoding::kUTF8, 10, 24,
                         font, paint);

  sk_sp<SkImage> img(surface->makeImageSnapshot());
  if (!img) {
    cout << "error> img null" << endl;
    return;
  }

  sk_sp<SkData> png(img->encodeToData());
  if (!png) {
    cout << "error> png is null" << endl;
    return;
  }

  cout << "png size " << png->size() << endl;

  SkFILEWStream out(_path);
  (void)out.write(png->data(), png->size());
}
