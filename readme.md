Mksvg
=====

An Svg Printer for Rust



Changes:
--------

### 0.1.2

* now g and start methods create return a TransWrap object, writing to this will write as though belonging to the svg
* no longer need to call ```end``` or ```g_end``` as drop will write the close.
* If a "g" element is empty it will not be written
