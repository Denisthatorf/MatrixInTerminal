cmake_minimum_required(VERSION 3.5)

project(ncurves LANGUAGES C)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_COMPILER "clang")


file(GLOB SOURCES_C "src/*.c" "src/*/*.c" "src/*/*/*.c" "src/*/*/*/*.c")

add_executable(ncurves ${SOURCES_C})

target_include_directories (ncurves
			PUBLIC 
				${CMAKE_SOURCE_DIR}/src/common)

target_link_libraries(ncurves
			PUBLIC	
				-lncurses)
