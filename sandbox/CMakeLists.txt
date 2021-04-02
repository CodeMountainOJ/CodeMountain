cmake_minimum_required(VERSION 3.5)

project(codemountain_sandbox)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -lpthread -Wall -Werror -std=c++20")

file(GLOB source_files "${PROJECT_SOURCE_DIR}/*.cpp")

add_executable(codemountain_sandbox ${source_files})
target_link_libraries(codemountain_sandbox seccomp)