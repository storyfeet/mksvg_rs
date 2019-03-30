Mksvg
=====

An Svg Printer for Rust



Changes:
--------

### 0.1.3

* Added Two new major types.  The Tag, and Text, these can now be build and finished with a write on an SvgWrite   eg: ```Tag::rect(x,y,w,h).fill("red").write<S:SvgWrite>(s:S);```

* Text also has a special method called bg which adds a a copy of the item behind it, to allow for wide strokes that don't cover the inside. 

### 0.1.2

* now g and start methods create return a TransWrap object, writing to this will write as though belonging to the svg
* no longer need to call ```end``` or ```g_end``` as drop will write the close.
* If a "g" element is empty it will not be written
