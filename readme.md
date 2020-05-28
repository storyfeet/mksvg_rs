Mksvg
=====

An Svg Printer for Rust



Changes:
--------

### 0.3.0 
Now uses anyhow and thiserror instead of failure and failure\_derive

### 0.2.1

Major change, all tags now need to be added using the Tag::tagname feature. This means that SvgWrite is no longer reliant on Generics and can be a Dynamic type.

To wrap a tag use 

```rust
let mut inner_writer = Tag::g().transform(3,4).wrap(outer_writer);
Tag::rect(5,6,100,100).write(&mut inner_writer);

```
when ```inner_writer``` is dropped, ```outer_writer``` will be writable again, ensuring the structure of the Svg is correct


The Page drawing library has now also changed to use a Builder Pattern.

eg
```rust

let files = page::Pages::build(cards).page_size(400,500).grid_size(4,4).write_pages("base/path_");
```


### 0.1.3

* Added Two new major types.  The Tag, and Text, these can now be build and finished with a write on an SvgWrite   eg: ```Tag::rect(x,y,w,h).fill("red").write<S:SvgWrite>(s:S);```

* Text also has a special method called bg which adds a a copy of the item behind it, to allow for wide strokes that don't cover the inside. 

### 0.1.2

* now g and start methods create return a TransWrap object, writing to this will write as though belonging to the svg
* no longer need to call ```end``` or ```g_end``` as drop will write the close.
* If a "g" element is empty it will not be written
