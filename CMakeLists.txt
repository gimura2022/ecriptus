cmake_minimum_required(VERSION 3.5)
project(ecripus C)

add_compile_options(-g)
add_compile_options(-fPIE)
add_compile_options(-Wall)
add_compile_options(-Wextra)

set(SRC_PATH  ${CMAKE_SOURCE_DIR})
set(SRC       "${SRC_PATH}/src")
set(INCLUDE   "${SRC_PATH}/include")

include_directories(${INCLUDE})
add_executable(${PROJECT_NAME} ${SRC}/main.c)

set(RUST_PATH "${CMAKE_SOURCE_DIR}/rust")
set(RUST_LIB_NAME "libecriptus_rust")

set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/rust)

include(ExternalProject)

ExternalProject_Add(
    rust
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build COMMAND cargo build --release
    BINARY_DIR "${RUST_PATH}"
    INSTALL_COMMAND ""
    LOG_BUILD ON
)

add_dependencies(${PROJECT_NAME} rust)

if(WIN32)
    target_link_libraries(${PROJECT_NAME}
        debug "${RUST_PATH}/target/debug/${RUST_LIB_NAME}.a"
        optimized "${RUST_PATH}/target/release/${RUST_LIB_NAME}.a"
        ws2_32 userenv advapi32 ntdll
    )
else()
    target_link_libraries(${PROJECT_NAME}
        debug "${RUST_PATH}/target/debug/${RUST_LIB_NAME}.a"
        optimized "${RUST_PATH}/target/release/${RUST_LIB_NAME}.a"
        m
    )
endif()