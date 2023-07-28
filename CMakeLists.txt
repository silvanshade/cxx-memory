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
target_compile_definitions(cxx-memory-abi PUBLIC LLVM_DISABLE_ABI_BREAKING_CHECKS_ENFORCING)
target_include_directories(cxx-memory-abi PUBLIC
  ../cxx-memory-abi/..
  cxx/include
  target/cxxbridge
  target/cxxbridge/rust
  ../swift-project/build/Ninja-ReleaseAssert/llvm-linux-x86_64/include
  ../swift-project/build/Ninja-ReleaseAssert/llvm-linux-x86_64/tools/clang/include
  ../swift-project/build/Ninja-ReleaseAssert/swift-linux-x86_64/include
  ../swift-project/llvm-project/clang/include
  ../swift-project/llvm-project/llvm/include
  ../swift-project
  ../swift-project/swift/include
  ../swift-project/swift/stdlib/public/SwiftShims
)
target_compile_options(cxx-memory-abi PUBLIC
  -std=gnu++2b
  # -stdlib=libc++
  # -lc++abi
  -Werror
  -Wall
  -Wextra
  -pedantic
  -Wno-ambiguous-reversed-operator
  -Wno-deprecated-anon-enum-enum-conversion
  -Wno-deprecated-builtins
  -Wno-dollar-in-identifier-extension
  -Wno-unused-parameter
  -fno-rtti # needed to avoid "undefined reference to `typeinfo for [...]`" errors
)
