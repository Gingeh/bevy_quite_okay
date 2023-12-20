#![warn(missing_docs)]

//! Plugins adding support for loading [QOI](https://qoiformat.org/) and [QOA](https://qoaformat.org/) assets in Bevy.
//!
//! # Usage
//! To use these asset loaders, just add the corresponding plugins to your app:
//! ```no_run
//! use bevy_quite_okay::{qoa, qoi};
//! # use bevy::prelude::*;
//! #
//! # let mut app = App::new();
//! # app.add_plugins(DefaultPlugins);
//!
//! app.add_plugins(qoa::QoaPlugin)
//!    .add_plugins(qoi::QoiPlugin);
//! ```
//! And use the `AssetServer` to load your files:
//! ```
//! # use bevy::prelude::*;
//! # use bevy_quite_okay::{qoa, qoi};
//! #
//! # fn load_things(mut commands: Commands, asset_server: Res<AssetServer>) {
//! commands.spawn(SpriteBundle {
//!     texture: asset_server.load("image.qoi"),
//!     ..Default::default()
//! });
//!
//! commands.spawn(AudioSourceBundle {
//!     source: asset_server.load::<qoa::QoaSource>("audio.qoa"),
//!     ..Default::default()
//! });
//! # }
//! ```
//!
//! ## Note for audio
//! In the example for loading a QOA file you may notice that the [`qoa::QoaSource`] type needed to be specified.
//! This is because custom audio decoders need to be implemented as separate types from the built-in `AudioSource` type.
//!
//! To play a QOA file you need to use the generic `AudioSourceBundle` (not `AudioBundle`!),
//! and specify the type if rust can't infer it.
//! If you want to save the loaded sound for later then you should store a `Handle<qoa::QoaSource>`


#[cfg(feature = "qoa")]
/// Support for loading QOA files.
pub mod qoa;

#[cfg(feature = "qoi")]
/// Support for loading QOI files.
pub mod qoi;
