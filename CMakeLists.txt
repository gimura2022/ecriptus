cmake_minimum_required(VERSION 3.5)
project(ecripus C)

add_compile_options(-g)
add_compile_options(-fPIE)
add_compile_options(-Wall)
add_compile_options(-Wextra)

set(SRC_PATH ${PROJECT_SOURCE_DIR})
set(SRC      "${SRC_PATH}/src")
set(INCLUDE  "${SRC_PATH}/include")

set(SRC_FILES "${SRC}/main.c")

include_directories(${INCLUDE})
add_executable(main ${SCR_FILES})