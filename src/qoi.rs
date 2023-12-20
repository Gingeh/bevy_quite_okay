use bevy::{
    app::{App, Plugin},
    asset::{io::Reader, AssetApp, AssetLoader, AsyncReadExt, LoadContext},
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::Image,
    },
    utils::BoxedFuture,
};
use rapid_qoi::Qoi;

/// Add this plugin to enable loading .qoi files.
pub struct QoiPlugin;

impl Plugin for QoiPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(QoiLoader);
    }
}

struct QoiLoader;

impl AssetLoader for QoiLoader {
    type Asset = Image;

    type Settings = ();

    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut qoi_data = Vec::new();
            reader.read_to_end(&mut qoi_data).await?;
            let (header, decoded) = Qoi::decode_alloc(&qoi_data)?;

            Ok(Image::new(
                Extent3d {
                    width: header.width,
                    height: header.height,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                decoded,
                match header.colors {
                    rapid_qoi::Colors::Rgba => TextureFormat::Rgba8Unorm,
                    rapid_qoi::Colors::SrgbLinA => TextureFormat::Rgba8UnormSrgb,
                    _ => anyhow::bail!("RGB images aren't supported"),
                },
            ))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["qoi"]
    }
}
