#pragma once

#include "sys/types.h"

#include <memory>
#include <type_traits>

// NOLINTBEGIN(google-runtime-int)
using c_char = char;
using c_int = int;
using c_long = long;
using c_longlong = long long;
using c_schar = signed char;
using c_short = short;
using c_uchar = unsigned char;
using c_uint = unsigned int;
using c_ulong = unsigned long;
using c_ulonglong = unsigned long long;
using c_ushort = unsigned short;
using c_void = void;
using c_off_t = off_t;
using c_time_t = time_t;
// NOLINTEND(google-runtime-int)

namespace cxx_memory::abi {
template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_align() noexcept -> size_t
{
  return alignof(T);
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_size() noexcept -> size_t
{
  return sizeof(T);
}

template<typename T, typename... Args>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_constructible() noexcept -> bool
{
  return ::std::is_constructible_v<T, Args...>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_default_constructible() noexcept -> bool
{
  return ::std::is_default_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_copy_constructible() noexcept -> bool
{
  return ::std::is_copy_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_move_constructible() noexcept -> bool
{
  return ::std::is_move_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_destructible() noexcept -> bool
{
  return ::std::is_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_copyable() noexcept -> bool
{
  return ::std::is_trivially_copyable_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_movable() noexcept -> bool
{
  return ::std::is_trivially_move_constructible_v<T> && ::std::is_trivially_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_destructible() noexcept -> bool
{
  return ::std::is_trivially_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_cxx_extern_type_trivial() noexcept -> bool
{
  return cxx_is_trivially_movable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_unpin() noexcept -> bool
{
  return cxx_is_trivially_movable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_send() noexcept -> bool
{
  return false;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_sync() noexcept -> bool
{
  return false;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_drop() noexcept -> bool
{
  return cxx_is_destructible<T>() && !cxx_is_trivially_destructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_copy() noexcept -> bool
{
  return cxx_is_trivially_copyable<T>() && cxx_is_trivially_movable<T>() && !rust_should_impl_drop<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_default() noexcept -> bool
{
  return cxx_is_default_constructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_cxx_memory_copy_new() noexcept -> bool
{
  return cxx_is_copy_constructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_cxx_memory_move_new() noexcept -> bool
{
  return cxx_is_move_constructible<T>();
}

} // namespace cxx_memory::abi

namespace cxx_memory::abi {
template<typename T, typename... Args>
[[gnu::always_inline]]
static inline auto
cxx_placement_new(T* This [[clang::lifetimebound]], Args&&... args) noexcept -> void
{
  if constexpr (cxx_is_constructible<T, Args...>()) {
    new (This) T(::std::forward<Args>(args)...);
  }
}

template<typename T>
[[gnu::always_inline]]
static inline auto
cxx_default_new(T* This [[clang::lifetimebound]]) noexcept -> void
{
  if constexpr (cxx_is_default_constructible<T>()) {
    cxx_placement_new(This);
  }
}

template<typename T>
[[gnu::always_inline]]
static inline auto
cxx_copy_new(T* This [[clang::lifetimebound]], T const& that [[clang::lifetimebound]]) noexcept -> void
{
  if constexpr (cxx_is_copy_constructible<T>() && ::std::is_lvalue_reference_v<decltype(that)>) {
    new (This) T(that);
  }
}

template<typename T>
[[gnu::always_inline]]
static inline auto
cxx_move_new(T* This [[clang::lifetimebound]], T&& that [[clang::lifetimebound]]) noexcept -> void
{
  if constexpr (cxx_is_move_constructible<T>() && ::std::is_rvalue_reference_v<decltype(that)>) {
    new (This) T(::std::forward<T>(that));
  }
}

template<typename T>
[[gnu::always_inline]]
static inline auto
cxx_destruct(T* This [[clang::lifetimebound]]) -> void
{
  if constexpr (cxx_is_destructible<T>()) {
    ::std::destroy_at(This);
  }
}

}; // namespace cxx_memory::abi

// NOLINTBEGIN(cppcoreguidelines-macro-usage, bugprone-macro-parentheses)
#define CXX_MEMORY_ABI_PRELUDE_SELECT_MACRO(_0, _1, _2, NAME, ...) NAME

#define CXX_MEMORY_ABI_PRELUDE_TY_CON_DEFINE_1(TY_CON) using TyCon = TY_CON;
#define CXX_MEMORY_ABI_PRELUDE_TY_CON_DEFINE_2(TY_CON, TY_ARG0)                                                        \
  template<typename TyArg0>                                                                                            \
  using TyCon = TY_CON<TyArg0>;
#define CXX_MEMORY_ABI_PRELUDE_TY_CON_DEFINE_3(TY_CON, TY_ARG0, TY_ARG1)                                               \
  template<typename TyArg0, typename TyArg1>                                                                           \
  using TyCon = TY_CON<TyArg0, TyArg1>;
#define CXX_MEMORY_ABI_PRELUDE_TY_CON_DEFINE(...)                                                                      \
  CXX_MEMORY_ABI_PRELUDE_SELECT_MACRO(                                                                                 \
    __VA_ARGS__,                                                                                                       \
    CXX_MEMORY_ABI_PRELUDE_TY_CON_DEFINE_3,                                                                            \
    CXX_MEMORY_ABI_PRELUDE_TY_CON_DEFINE_2,                                                                            \
    CXX_MEMORY_ABI_PRELUDE_TY_CON_DEFINE_1,                                                                            \
  )                                                                                                                    \
  (__VA_ARGS__)

#define CXX_MEMORY_ABI_PRELUDE_TY_ARGS_DEFINE_1(TY_CON)
#define CXX_MEMORY_ABI_PRELUDE_TY_ARGS_DEFINE_2(TY_CON, TY_ARG0) using TyArg0 = TY_ARG0;
#define CXX_MEMORY_ABI_PRELUDE_TY_ARGS_DEFINE_3(TY_CON, TY_ARG0, TY_ARG1)                                              \
  using TyArg0 = TY_ARG0;                                                                                              \
  using TyArg1 = TY_ARG1;
#define CXX_MEMORY_ABI_PRELUDE_TY_ARGS_DEFINE(...)                                                                     \
  CXX_MEMORY_ABI_PRELUDE_SELECT_MACRO(                                                                                 \
    __VA_ARGS__,                                                                                                       \
    CXX_MEMORY_ABI_PRELUDE_TY_ARGS_DEFINE_3,                                                                           \
    CXX_MEMORY_ABI_PRELUDE_TY_ARGS_DEFINE_2,                                                                           \
    CXX_MEMORY_ABI_PRELUDE_TY_ARGS_DEFINE_1,                                                                           \
  )                                                                                                                    \
  (__VA_ARGS__)

#define CXX_MEMORY_ABI_PRELUDE_TYPE_DEFINE_1(TY_CON) using Self = TY_CON;
#define CXX_MEMORY_ABI_PRELUDE_TYPE_DEFINE_2(TY_CON, TY_ARG0) using Self = TY_CON<TY_ARG0>;
#define CXX_MEMORY_ABI_PRELUDE_TYPE_DEFINE_3(TY_CON, TY_ARG0, TY_ARG1) using Self = TY_CON<TY_ARG0, TY_ARG1>;

#define CXX_MEMORY_ABI_PRELUDE_TYPE_DEFINE(...)                                                                        \
  CXX_MEMORY_ABI_PRELUDE_SELECT_MACRO(                                                                                 \
    __VA_ARGS__,                                                                                                       \
    CXX_MEMORY_ABI_PRELUDE_TYPE_DEFINE_3,                                                                              \
    CXX_MEMORY_ABI_PRELUDE_TYPE_DEFINE_2,                                                                              \
    CXX_MEMORY_ABI_PRELUDE_TYPE_DEFINE_1,                                                                              \
  )                                                                                                                    \
  (__VA_ARGS__)

#define CXX_MEMORY_ABI_PRELUDE(CXX_NAME, ...)                                                                          \
  CXX_MEMORY_ABI_PRELUDE_TY_CON_DEFINE(__VA_ARGS__)                                                                    \
  CXX_MEMORY_ABI_PRELUDE_TY_ARGS_DEFINE(__VA_ARGS__)                                                                   \
  CXX_MEMORY_ABI_PRELUDE_TYPE_DEFINE(__VA_ARGS__)                                                                      \
  using CXX_NAME = Self;                                                                                               \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_abi_align() noexcept -> size_t                                                      \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_abi_align<Self>();                                                                     \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_abi_size() noexcept -> size_t                                                       \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_abi_size<Self>();                                                                      \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_default_constructible() noexcept -> bool                                         \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_is_default_constructible<Self>();                                                      \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_copy_constructible() noexcept -> bool                                            \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_is_copy_constructible<Self>();                                                         \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_move_constructible() noexcept -> bool                                            \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_is_move_constructible<Self>();                                                         \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_destructible() noexcept -> bool                                                  \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_is_destructible<Self>();                                                               \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_trivially_copyable() noexcept -> bool                                            \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_is_trivially_copyable<Self>();                                                         \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_trivially_movable() noexcept -> bool                                             \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_is_trivially_movable<Self>();                                                          \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_trivially_destructible() noexcept -> bool                                        \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_is_trivially_destructible<Self>();                                                     \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_cxx_extern_type_trivial() noexcept -> bool                             \
  {                                                                                                                    \
    return cxx_memory::abi::rust_should_impl_cxx_extern_type_trivial<Self>();                                          \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_unpin() noexcept -> bool                                               \
  {                                                                                                                    \
    return cxx_memory::abi::rust_should_impl_unpin<Self>();                                                            \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_send() noexcept -> bool                                                \
  {                                                                                                                    \
    return cxx_memory::abi::rust_should_impl_send<Self>();                                                             \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_sync() noexcept -> bool                                                \
  {                                                                                                                    \
    return cxx_memory::abi::rust_should_impl_sync<Self>();                                                             \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_drop() noexcept -> bool                                                \
  {                                                                                                                    \
    return cxx_memory::abi::rust_should_impl_drop<Self>();                                                             \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_copy() noexcept -> bool                                                \
  {                                                                                                                    \
    return cxx_memory::abi::rust_should_impl_copy<Self>();                                                             \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_default() noexcept -> bool                                             \
  {                                                                                                                    \
    return cxx_memory::abi::rust_should_impl_default<Self>();                                                          \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_cxx_memory_copy_new() noexcept -> bool                                 \
  {                                                                                                                    \
    return cxx_memory::abi::rust_should_impl_cxx_memory_copy_new<Self>();                                              \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_cxx_memory_move_new() noexcept -> bool                                 \
  {                                                                                                                    \
    return cxx_memory::abi::rust_should_impl_cxx_memory_move_new<Self>();                                              \
  }                                                                                                                    \
                                                                                                                       \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_default_new(Self* This [[clang::lifetimebound]]) noexcept -> void                             \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_default_new(This);                                                                     \
  }                                                                                                                    \
                                                                                                                       \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_copy_new(                                                                                     \
    Self* This [[clang::lifetimebound]], Self const& that [[clang::lifetimebound]]                                     \
  ) noexcept -> void                                                                                                   \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_copy_new(This, that);                                                                  \
  }                                                                                                                    \
                                                                                                                       \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_move_new(Self* This [[clang::lifetimebound]], Self* that [[clang::lifetimebound]]) noexcept   \
    -> void                                                                                                            \
  {                                                                                                                    \
    /* NOLINTNEXTLINE(hicpp-move-const-arg, performance-move-const-arg) */                                             \
    Self&& that_rvalue = ::std::move(*that);                                                                           \
    return cxx_memory::abi::cxx_move_new(This, ::std::forward<Self&&>(that_rvalue));                                   \
  }                                                                                                                    \
                                                                                                                       \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_destruct(Self* This [[clang::lifetimebound]])                                                 \
    ->void                                                                                                             \
  {                                                                                                                    \
    return cxx_memory::abi::cxx_destruct(This);                                                                        \
  }

// NOLINTEND(cppcoreguidelines-macro-usage, bugprone-macro-parentheses)
