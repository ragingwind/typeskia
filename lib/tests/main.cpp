#include <filesystem>
#include <iostream>
#include <gtest/gtest.h>

#include "../src/typeskia.h"

namespace fs = std::filesystem;

int main(int argc, char *argv[]) {
  auto font_root =
      fs::weakly_canonical(fs::path(argv[0]) / "../../lib/fonts");
  std::cout << "font_root: " << font_root << std::endl;

  draw(200, 200, "A", font_root.c_str(), "./test.png");

  ::testing::InitGoogleTest(&argc, argv);

  return RUN_ALL_TESTS();
}
