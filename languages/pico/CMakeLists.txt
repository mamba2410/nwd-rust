cmake_minimum_required(VERSION 3.12)

## Include pico cmake file
include(pico_sdk_import.cmake)

## Set project name
project(PROJECT_NAME)

## Init pico CMake stuff
pico_sdk_init()

## Create executable from dependencies
add_executable(PROJECT_NAME
	src/PROJECT_NAME.c
)

## Enable usb, uart etc for stdio
pico_enable_stdio_usb(PROJECT_NAME 1)
pico_enable_stdio_uart(PROJECT_NAME 0)

## Add outputs other than .elf file
pico_add_extra_outputs(PROJECT_NAME)

## Link these libs together
target_link_libraries(PROJECT_NAME
	pico_stdlib
)
