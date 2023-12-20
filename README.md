# bevy_quite_okay
Plugins adding support for loading QOI and QOA assets in Bevy.

# Usage
To use these asset loaders, just add the corresponding plugins to your app:
```rust
use bevy_quite_okay::{qoa, qoi};

app.add_plugins(qoa::QoaPlugin)
   .add_plugins(qoi::QoiPlugin);
```
And use the `AssetServer` to load your files:
```rust
commands.spawn(SpriteBundle {
    texture: asset_server.load("image.qoi"),
    ..Default::default()
});

commands.spawn(AudioSourceBundle {
    source: asset_server.load::<qoa::QoaSource>("audio.qoa"),
    ..Default::default()
});
```

## Note for audio
In the example for loading a QOA file you may notice that the `qoa::QoaSource` type needed to be specified.
This is because custom audio decoders need to be implemented as separate types from the built-in `AudioSource` type.

To play a QOA file you need to use the generic `AudioSourceBundle` (not `AudioBundle`!),
and specify the type if rust can't infer it.
If you want to save the loaded sound for later then you should store a `Handle<qoa::QoaSource>`
