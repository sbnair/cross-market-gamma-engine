cmake_minimum_required(VERSION 3.16)
project(cross_market_gamma_engine CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(include)

add_library(core
    src/pricing.cpp
    src/execution.cpp
)

add_executable(demo src/main.cpp)
target_link_libraries(demo PRIVATE core)
