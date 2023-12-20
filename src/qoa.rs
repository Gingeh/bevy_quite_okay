use std::{io::Cursor, sync::Arc};

use bevy::{
    app::{App, Plugin},
    asset::{io::Reader, Asset, AssetApp, AssetLoader, AsyncReadExt, LoadContext},
    audio::{AddAudioSource, Decodable},
    reflect::TypePath,
    utils::BoxedFuture,
};
use qoaudio::{QoaDecoder, QoaRodioSource};

/// Add this plugin to enable loading .qoa files.
pub struct QoaPlugin;

impl Plugin for QoaPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(QoaLoader)
            .add_audio_source::<QoaSource>();
    }
}

struct QoaLoader;

impl AssetLoader for QoaLoader {
    type Asset = QoaSource;

    type Settings = ();

    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut qoa_data = Vec::new();
            reader.read_to_end(&mut qoa_data).await?;
            Ok(QoaSource(Arc::new(qoa_data)))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["qoa"]
    }
}

/// A source of decodable QOA data.
///
/// Use this type instead of `AudioSource`.
#[derive(Asset, TypePath, Clone)]
pub struct QoaSource(Arc<Vec<u8>>);

impl AsRef<[u8]> for QoaSource {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Decodable for QoaSource {
    type DecoderItem = i16;
    type Decoder = QoaRodioSource<Cursor<Self>>;

    fn decoder(&self) -> Self::Decoder {
        let decoder = QoaDecoder::new(Cursor::new(self.clone())).unwrap();
        QoaRodioSource::new(decoder)
    }
}
