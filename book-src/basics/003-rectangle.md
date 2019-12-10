# Rectangle Elements

Naturally we don't want just one triangle. When you're playing The Witcher 3,
there's at least two triangles on the screen (maybe more!).

Let's move on to drawing a rectangle. For this we need a second triangle.

We could just add three more vertex entries and call it a day. If we wanted two
triangles that were each on their own that's what we might do. However, since
these two triangles making up our rectangle are going to be directly touching,
that means we'd have six vertexes making up only four "real" points. That's 50%
more space used than we want! It may seem small now, but a complete model for a
tree or a person or something like that can easily end up being thousands of
triangles. Making that be 50% more space used is a bad time.

Of course this problem of duplicate vertices is a fairly easy problem to solve,
and GL has it covered. What we do is specify an [Index
Buffer](https://www.khronos.org/opengl/wiki/Vertex_Specification#Index_buffers).
It holds the indexes of the vertex buffer entries we want to use to form each
geometry element (in this case triangles). Then the vertex buffer doesn't need
to have any duplicates, we just have more than one triangle index the same
vertex.

Note: What we'll be drawing is usually called a "quad", because the important
part is that it has four outside edges. It's not really important that the edges
are in two pairs of parallel lines at right angles with each other like a true
rectangle has.

## Data

So we've got some new data. We're going to have 4 vertex entries that describes
the points we want to use, and an index buffer with 2 entries where each entry
describes a triangle using the points.

```rust
type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] =
  [[0.5, 0.5, 0.0], [0.5, -0.5, 0.0], [-0.5, -0.5, 0.0], [-0.5, 0.5, 0.0]];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];
```

## Element Buffer Object

Our indexes go into a separate kind of buffer. This is that `ElementArray`
buffer type that I snuck into the cleanup lesson.

After we make and bind our vertex data we also bind a buffer for the element
data and upload it, the code looks nearly identical:

```rust
let ebo = Buffer::new().expect("Couldn't make the element buffer.");
ebo.bind(BufferType::ElementArray);
learn::buffer_data(
  BufferType::ElementArray,
  bytemuck::cast_slice(&INDICES),
  GL_STATIC_DRAW,
);
```

## Draw It!

Finally, instead of calling `glDrawArrays`, we use a separate function called [`glDrawElements`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElements.xhtml).

* `mode`: The style of drawing. We're still drawing triangles so we keep that
  from before.
* `count`: The number of index elements to draw. We want two triangles to form
  our quad, and there's three indexes per triangle, so we put 6.
* `type`: This is the type of the index data. The `u32` type is specified with
  `GL_UNSIGNED_INT`. I used `u32` out of habit, we could have made our indexes
  be `u16` or `u8` as well.
* `indices`: is a pointer to the position within the index buffer to start the
  drawing with. Similar to the attribute specification, you pretend the index
  buffer starts at address 0 and then you decide the offset you want, and then
  cast that to a `*const` pointer.

So the usage looks like this:

```rust
// and then draw!
unsafe {
  glClear(GL_COLOR_BUFFER_BIT);
  glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *const _);
}
win.swap_window();
```

## Bonus: Wireframe Mode

Since this lesson is really short let's look at one extra ability we can use.

You often see 3d models with just the outlines of each triangle. "Wireframe
mode" it's sometimes called. We can easily do that with
[`glPolygonMode`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonMode.xhtml).

* We can specify the `face`, but in the Core profile the only valid value is
  `GL_FRONT_AND_BACK` (in Compatibility profile you can also use `GL_FRONT` or
  `GL_BACK`).
* We also specify the `mode`. The default is `GL_FILL`, but With `GL_LINE` we
  get the wireframe effect. `GL_POINT` is also allowed, but makes it pretty hard
  to see what's going on.

All this can go in our `lib.rs` file:

```rust
/// The polygon display modes you can set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
  /// Just show the points.
  Point = GL_POINT as isize,
  /// Just show the lines.
  Line = GL_LINE as isize,
  /// Fill in the polygons.
  Fill = GL_FILL as isize,
}

/// Sets the font and back polygon mode to the mode given.
pub fn polygon_mode(mode: PolygonMode) {
  unsafe { glPolygonMode(GL_FRONT_AND_BACK, mode as GLenum) };
}
```

And then before our main loop we can turn it on:

```rust
learn::polygon_mode(PolygonMode::Line);
```

Now we get a wireframe quad! and it looks like two triangles just like it should!

## Done!

* Code [003-rectangle-elements](https://github.com/rust-tutorials/learn-opengl/blob/master/examples/003-rectangle-elements.rs)
