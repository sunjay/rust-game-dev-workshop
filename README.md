# rust-game-dev-workshop

This is a minimally "complete" game created in Rust. It has a
keyboard-controlled player, some enemies/obstacles, and a goal to reach. It's
hard to define what exactly makes a game "complete", but these three things
together do at least make something you can play.

![Minimal Rust Game](preview.gif)

This game was created for use in my RustConf 2019 Game Development Workshop.
There are two separate implementations: one without the specs ECS and one that
uses it.

## Acknowledgements

The sources of the assets used in this game are linked below:

* [Bard Character](http://finalbossblues.com/timefantasy/freebies/bard-character-with-animations/)
* [Grim Reaper](http://finalbossblues.com/timefantasy/freebies/grim-reaper-sprites/)
* [Trees](http://finalbossblues.com/timefantasy/freebies/more-trees/)
* [Pink Trees](http://finalbossblues.com/timefantasy/freebies/pink-trees/)

These are all free assets (see each link for the specific licencing details).
Please consider supporting the artist(s) by buying some of their paid assets.
Game development would be much more difficult without them and their work.

## Limitations

Since this game was created for the purposes of a workshop, I intentionally
chose to leave the code as small and simple as possible. Game development is an
area with an enormous amount of detail and nuance. Trying to cover all of the
subtle edge cases would have increased the amount of code significantly.

If you intend to build on this code and create your game, it's a good idea to be
aware of some of these limitations so you can address them as needed. The
following is a **non-exhaustive** list (in no particular order) of some of the
items you may want to resolve:

* We use `thread::sleep` to maintain a roughly 60 FPS timestep. There are many
  potential issues with this, one of which is the fact that sleep is not
  guaranteed to last for exactly the duration you specify. It can go longer if
  the processor is busy. That means that your game can quickly begin to lag.
  There are far better approaches described in various sources (e.g.
  [*Fix Your Timestep!*](http://web.archive.org/web/20190506122532/http://gafferongames.com/post/fix_your_timestep/)).
* The game currently requires you to press and release one arrow key at a time.
  If you accidentally start holding another arrow key before you fully release
  the other you will end up stopping the character accidentally. A better
  approach is to maintain a stack of pressed directions. You can then push each
  direction as it is pressed and pop when it is released. This will even let you
  continue in a previously held direction when you are still holding it after
  other directions have been released.
* We use `usize` directly to refer to items in the textures array, but you may
  be able to achieve some more type safety (and maybe performance) using a
  separate `TextureId(usize)` type. The following is a skeleton of the code you
  might use to do this. The added performance can come from using the `unsafe`
  method `get_unchecked` instead of indexing given that you can guarantee that
  the index is valid (as long as you keep that field of `TextureId` private).
  ```rust
  pub struct TextureId(usize);

  pub struct Textures<'a> {
      textures: Vec<Texture<'a>>,
  }

  impl<'a> Textures<'a> {
      pub fn new() -> Self {
        Self { textures: Vec::new() }
      }

      pub fn add(&mut self, texture: Texture<'a>) -> TextureId {
        self.textures.push(texture)
        TextureId(self.textures.len() - 1)
      }

      pub fn get(&mut self, TextureId(id): TextureId) -> &Texture<'a> {
        // Can be made more performant with `get_unchecked`
        &self.textures[id]
      }
  }
  ```
* In the code without the ECS, the current animation frame does not reset when
  changing directions. This is completely unnoticeable for walking animations,
  but could potentially matter for other things (e.g. attacking).
* When the character stops moving, they just kind of freeze in place (no matter
  which frame of the walking animation they were on). That means that they can
  stop mid-stride. This is sort of awkward and that's why most spritesheets
  include some sort of "idle" frame where the character is in a neutral
  position. It's a good idea to return the character to that neutral position
  when they stop moving. Some spritesheets will even include an idle *animation*
  that you can play after a few seconds if the character isn't moving anymore.
* Animations are not guaranteed to advance even a single frame. That means that
  if you tap/feather an arrow key, you can make a character float from point to
  point without their step ever being animated.
* Providing a duration for each frame is a fairly typical approach for
  specifying sprite animations. It has a downside though that you need to make
  sure you tune it to the walking speed. If you don't, it can appear like the
  character is floating or otherwise "walking on air". One potential way to deal
  with this (at least for walking) is to compute the frame durations based on
  the stride length. For example, if you know the speed in pixels/second and you
  know that the character's stride has a given length (in pixels) and takes 2
  frames, you can calculate how long the frame duration should be so that their
  animation moves with the right pace based on their speed.
* In the code without the ECS, it isn't possible to specify durations for each
  frame individually. This is fine in that example because we only have walking
  animations and all of the frames last the same amount of time, but you would
  really want something more robust in an actual game. Speeding up and slowing
  down animations at certain points (rather than just using the same fixed
  duration for each frame) can really change the "feel" of your game and impact
  how immersive the movement is.
* The game uses only the 2x variants of each asset because otherwise everything
  becomes way too small on high DPI displays. A better solution would be to
  properly use the concepts of "logical size" vs. "output size". The logical
  size should always be approximately constant so that the calculations in your
  game remain consistent. You can then use the output size set to some multiple
  of the logical size to scale things up for high DPI displays. It's a good idea
  to sample images/textures based on the scale factor you use. For example, if
  your output size is 2 times your logical size (for high DPI displays), sample
  the 2x textures. The result that you'll observe is that your game will look
  like it's the same size on different displays but it'll still be able to take
  advantage of the full resolution of the display it is currently on. It's
  tricky to get this completely correct which is why we don't deal with it at
  all in this code. There is lots of information online about this.
* As the rendering code becomes more complex, it'll be even more important to
  make sure you abstract the conversion from world to screen coordinates. One
  possible way to do this is to create a wrapper around the canvas methods that
  always performs the conversion. You want to find a way to structure your code
  so you can never forget to convert a point/rect given in world coordinates to
  a point/rect in screen coordinates.
* Everything in the game is hard-coded as much as possible. That includes the
  sizes of the bounding box, the sizes of the frames, animation lengths, etc.
  This makes the code fairly brittle for if you ever decide to use other
  sprites or change the coordinate systems in any way. It's a good idea to be
  aware of the assumptions you are making and document them as much as possible.
  See if you can remove some of the hard-coded values by calculating them based
  on a configuration instead.
* We only use `VecStorage` (and `NullStorage`) for our components. This is
  incredibly wasteful and you should instead read the specs guide and make more
  informed decisions about the storage to use.
* Animations are currently automatically repeated once you reach the last frame.
  This isn't desirable for things like attack or hit animations. You should
  update the code to support non-repeating animations as well. (For the ECS, it
  is easy to just remove the non-repeating animation once it is complete.)
* We use a frame timer to change to the next frame once the frame duration has
  elapsed. By resetting the frame timer to `Instant::now()`, we actually create
  a subtle animation lag that will cause the next frame to potentially actually
  take slightly longer than its configured duration. This can happen because it
  is possible that `Instant::now() - anim.frame_timer > anim.frames[anim.current_frame].duration`.
  In that case, you should subtract the excess time from `Instant::now()` in
  order to make sure that each frame actually lasts only its configured amount.
  This is such a subtle bug that it is probably pretty hard to notice even if
  you stare at the character. The processor would have to be very busy with
  something else in order to make this actually visible. That being said, if
  this isn't resolved, then your frame duration is not *technically* guranteed
  to be the actual duration of the frame. Even if you don't decide to do
  anything about this, it's very important to understand the guarantees of your
  program.
* The bounding box of each entity in both versions of the code surrounds the
  center of the sprite. The problem with this is that the sprites may not align
  exactly with the bounding box. You can see this if you move the character to
  the very bottom of the window. If the code was working, the bottom of the
  character sprite would not get cut off as it does currently. The reason this
  is a problem is because it can affect the correctness of your code. The player
  can appear to not run into anything because their bounding box doesn't
  accurately wrap the sprite that is being rendered. The way to fix this is to
  actually add some sort of offset to the `Sprite` struct (in the ECS code) and
  add that when computing where to copy the sprite. You can then configure the
  offset on a per-sprite basis so that the character is always centered in their
  bounding box.

Given that the goal was to keep the code small, I will **not** be accepting pull
requests to fix any of these limitations. That being said, if you find something
very severe, feel free to open an issue or PR and we can talk about integrating
it. Contributions to the list above are welcome.
