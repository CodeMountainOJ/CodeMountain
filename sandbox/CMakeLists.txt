cmake_minimum_required(VERSION 3.5)

project(codemountain_sandbox C)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c11")

include_directories("${PROJECT_SOURCE_DIR}/vendor/include")

file(GLOB source_files "${PROJECT_SOURCE_DIR}/*.c")

add_executable(codemountain_sandbox ${source_files})