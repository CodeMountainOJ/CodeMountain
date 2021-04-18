cmake_minimum_required(VERSION 3.5)

project(codemountain_sandbox)

file(GLOB source_files "${PROJECT_SOURCE_DIR}/*.cpp")

if(DEBUG)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pthread -Wall -Werror -DDEBUGMODE")
else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pthread -Wall -Werror")
endif()

add_executable(codemountain_sandbox ${source_files})
target_link_libraries(codemountain_sandbox seccomp)

set_property(TARGET codemountain_sandbox PROPERTY CXX_STANDARD 20)
