# CMake find module for html-to-markdown-rs
get_filename_component(_dir "${CMAKE_CURRENT_LIST_FILE}" PATH)
get_filename_component(_prefix "${_dir}/../.." ABSOLUTE)

set(html-to-markdown-rs_INCLUDE_DIR "${_prefix}/include")
set(html-to-markdown-rs_LIBRARY "${_prefix}/lib/libhtml_to_markdown_ffi${CMAKE_SHARED_LIBRARY_SUFFIX}")

if(EXISTS "${html-to-markdown-rs_LIBRARY}")
  set(html-to-markdown-rs_FOUND TRUE)
else()
  set(html-to-markdown-rs_FOUND FALSE)
endif()
