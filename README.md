# cncsim

Simulates gcode from a cnc router. Converts gcode to a .stl or an image in a .png file.

# Purpose

If you want to see the output of a gcode using a cnc router without actually running it. This way should be faster as well.


# Usage

First you must change main.rs to get the right tools in. In the future I would like to do a .toml file for your tools.

```
cargo run -- --input path/to/file.nc --output image.png --blockwidth 10 --blockheight 10 --imgwidth 4096 --imgheight 4096 --fnvalue 100
```

Notice here that you currently still have to specify block / img size. In the future we should remove that and add in default values.
You also may not be able to do big gcode files. Any more than 500 lines may take a long time.
```
cargo run -- --input path/to/file.nc --output image.stl --blockwidth 10 --blockheight 10 --imgwidth 4096 --imgheight 4096 --fnvalue 3
```

# Examples

Used fuison 360 to create the gcode for outputfiles/fusionsign.png.
We used [CamCam](https://github.com/Monksc/camcam) to create the gcode for the other examples in outputfiles.
Then to create the .stl file and .png images we used this repo cncsim.

A big use case is to see the step over value and how flat the floor is. You can also check to make sure it got to every place it can.

Gcode below was created with [CamCam](https://github.com/Monksc/camcam) 

![Diamond created with camcam software](/outputfiles/camcamsign.png)

The gcode below was created with fusion 360.
![Square created with camcam software](/outputfiles/fusionsign.png)

Other software to view gcode is listed below

To view gcode path https://ncviewer.com/

To check for errors in software you can use https://vector76.github.io/gcode_tpgen/checker.html

# Known Issues

Due to this project not being that big currently it only cuts with G1 code and only flat lines below 0 axis. If you want to use this software elsewhere then
contact me by putting in a issue. I should get back within a few days. If not then you can email me or repeatly spam the issues till I respond.
