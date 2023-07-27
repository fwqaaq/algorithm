
add_includedirs("Array/include")

target("array")
  set_kind("static")
  add_files("Array/*.c")

target("test")
  set_group("test")
  set_kind("binary")
  add_files("Array/test/*.c")
  add_deps("array")
