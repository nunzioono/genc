cmake_minimum_required(VERSION 3.10)

# Ensure that the PROJECT_NAME variable is defined
if (NOT DEFINED PROJECT_NAME)
    message(FATAL_ERROR "You must specify a project name using -DPROJECT_NAME=<name>")
endif()

# Get and display the current CMake version
message(STATUS "Using CMake version ${CMAKE_VERSION}")

# Create project directories
file(MAKE_DIRECTORY ${PROJECT_NAME}/src)
file(MAKE_DIRECTORY ${PROJECT_NAME}/build)

# Create main.cpp file
file(WRITE ${PROJECT_NAME}/src/main.cpp "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, ${PROJECT_NAME}!\" << std::endl;\n    return 0;\n}\n")

# Create CMakeLists.txt file
file(WRITE ${PROJECT_NAME}/CMakeLists.txt "cmake_minimum_required(VERSION 3.10)\n\n# Set the project name\nproject(${PROJECT_NAME})\n\n# Add an executable\nadd_executable(${PROJECT_NAME} src/main.cpp)\n")

message(STATUS "Project ${PROJECT_NAME} scaffolded successfully!")
