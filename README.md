# GenC
A scaffolder for initializing cmake projects really fast

## Usage

NOTE! This setup requires you to already have installed cmake on your device.
Download the executable from the "Releases" section and run the command:

```genc PROJECT_NAME```

A project will be initialized with a new folder with the name passed to the command,
a src folder within it with a main.cpp file, then a CMakeList.txt and a build folder (empty):

PROJECT_NAME<br>
|- src<br>
|  |- main.cpp<br>
|<br>
|- build<br>
|- CMakeLists.txt


## Contributions

Contributions are really appreciated, enjoy your time and submit pull requests, remember to open issues or discussions if you have any doubt on how to contribute.
The project right now publishes new releases with github actions so remember to updated the version in the Cargo.toml file so that the action can detect a new version has to be published.

