# NOTE: run with `cmake -G Ninja -S . -B build && cmake --build build`

cmake_minimum_required(VERSION 3.22.1)

project(cxx-memory-abi LANGUAGES CXX)

# export CMake configuration to compile_commands.json for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# ensure that the C++ compiler is `clang++`
set(CMAKE_CXX_COMPILER "clang++")

add_library(cxx-memory-abi STATIC
  crates/cxx-memory-abi/cxx/lib/cmake.cxx
)
target_include_directories(cxx-memory-abi PUBLIC
  ../cxx-memory/crates/cxx-memory-abi/..
  target/cxxbridge
)
target_compile_definitions(cxx-memory-abi PUBLIC _LIBCPP_ENABLE_THREAD_SAFETY_ANNOTATIONS)
target_compile_options(cxx-memory-abi PUBLIC
  -std=gnu++20
  # -stdlib=libc++
  -Werror
  -Wall
  -Wextra
  -Wthread-safety
  -Wthread-safety-beta
  -pedantic
  -Wno-ambiguous-reversed-operator
  -Wno-deprecated-anon-enum-enum-conversion
  -Wno-deprecated-builtins
  -Wno-dollar-in-identifier-extension
  -Wno-unused-parameter
  -fno-rtti # needed to avoid "undefined reference to `typeinfo for [...]`" errors
)
