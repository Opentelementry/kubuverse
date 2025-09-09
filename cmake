cmake_minimum_required(VERSION 3.19)

# Project setup
project(runner LANGUAGES CXX)

# ──────────────────────────────
# Application target
# ──────────────────────────────
add_executable(${BINARY_NAME} WIN32
  "flutter_window.cpp"
  "main.cpp"
  "utils.cpp"
  "win32_window.cpp"
  "${FLUTTER_MANAGED_DIR}/generated_plugin_registrant.cc"
  "Runner.rc"
  "runner.exe.manifest"
)

# ──────────────────────────────
# Standard build settings
# ──────────────────────────────
apply_standard_settings(${BINARY_NAME})

# ──────────────────────────────
# Preprocessor definitions
# (Build version metadata injected at compile time)
# ──────────────────────────────
target_compile_definitions(${BINARY_NAME} PRIVATE
  FLUTTER_VERSION="${FLUTTER_VERSION}"
  FLUTTER_VERSION_MAJOR=${FLUTTER_VERSION_MAJOR}
  FLUTTER_VERSION_MINOR=${FLUTTER_VERSION_MINOR}
  FLUTTER_VERSION_PATCH=${FLUTTER_VERSION_PATCH}
  FLUTTER_VERSION_BUILD=${FLUTTER_VERSION_BUILD}
  NOMINMAX   # Disable Windows min/max macros
)

# ──────────────────────────────
# Link libraries
# ──────────────────────────────
target_link_libraries(${BINARY_NAME} PRIVATE
  flutter
  flutter_wrapper_app
  dwmapi.lib
)

# ──────────────────────────────
# Include directories
# ──────────────────────────────
target_include_directories(${BINARY_NAME} PRIVATE
  "${CMAKE_SOURCE_DIR}"
)

# ──────────────────────────────
# Dependencies
# ──────────────────────────────
# Ensure Flutter build rules run first
add_dependencies(${BINARY_NAME} flutter_assemble)

# ──────────────────────────────
# Optional: Install rules
# (Lets you run `cmake --install .`)
# ──────────────────────────────
install(TARGETS ${BINARY_NAME}
  RUNTIME DESTINATION bin
)
