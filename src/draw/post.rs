//! A custom post processing effect, using two cameras, with one reusing the render texture of the first one.
//! Here a chromatic aberration is applied to a 3d scene containing a rotating cube.
//! This example is useful to implement your own post-processing effect such as
//! edge detection, blur, pixelization, vignette... and countless others.

use bevy::{
  prelude::*,
  reflect::TypeUuid,
  render::{
      render_resource::{
          AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
          TextureUsages,
      },
      texture::BevyDefault,
      view::RenderLayers,
  },
  sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use super::app::SSPost;
pub struct RS98PostPlugin;

impl Plugin for RS98PostPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
      .add_startup_system_to_stage(SSPost, setup_post)
      .add_system(update_post);
  }
}

#[derive(Resource)]
pub struct RenderImage {
  pub image: Handle<Image>,
  pub material: Handle<PostProcessingMaterial>,
}

fn setup_post(
  mut commands: Commands,
  mut windows: ResMut<Windows>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
  mut images: ResMut<Assets<Image>>,
) {
  let window = windows.primary_mut();
  let size = Extent3d {
      width: window.physical_width(),
      height: window.physical_height(),
      ..default()
  };
    // let size = Extent3d {
    //     width: 800,
    //     height: 600,
    //     ..default()
    // };

  // This is the texture that will be rendered to.
  let mut image = Image {
      texture_descriptor: TextureDescriptor {
          label: None,
          size,
          dimension: TextureDimension::D2,
          format: TextureFormat::bevy_default(),
          mip_level_count: 1,
          sample_count: 1,
          usage: TextureUsages::TEXTURE_BINDING
              | TextureUsages::COPY_DST
              | TextureUsages::RENDER_ATTACHMENT,
      },
      ..default()
  };

  // fill image.data with zeroes
  image.resize(size);

  let image_handle = images.add(image);

  // Main camera, first to render
  /*commands.spawn((
      Camera3dBundle {
          camera_3d: Camera3d {
              clear_color: ClearColorConfig::Custom(Color::WHITE),
              ..default()
          },
          camera: Camera {
              target: RenderTarget::Image(image_handle.clone()),
              ..default()
          },
          transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
              .looking_at(Vec3::default(), Vec3::Y),
          ..default()
      },
      // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
      // the cost of rendering the UI without any post processing effects.
      UiCameraConfig { show_ui: false },
  ));*/

  // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
  let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

  let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
      size.width as f32,
      size.height as f32,
  ))));

  // This material has the texture that has been rendered.
  let material_handle = post_processing_materials.add(PostProcessingMaterial {
      source_image: image_handle.clone(),
      time: 0.0,
  });

  // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
  commands.spawn((
      MaterialMesh2dBundle {
          mesh: quad_handle.into(),
          material: material_handle.clone(),
          transform: Transform {
              translation: Vec3::new(0.0, 0.0, 1.5),
              ..default()
          },
          ..default()
      },
      post_processing_pass_layer,
  ));

  // The post-processing pass camera.
  commands.spawn((
      Camera2dBundle {
          camera: Camera {
              // renders after the first main camera which has default value: 0.
              priority: 1,
              hdr: true,
              ..default()
          },
          ..Camera2dBundle::default()
      },
      post_processing_pass_layer,
      UiCameraConfig { show_ui: false },
  ));

  commands.insert_resource(RenderImage { 
    image: image_handle,
    material: material_handle,
  });
}

// Region below declares of the custom material handling post processing effect

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct PostProcessingMaterial {
  /// In this example, this image will be the result of the main camera.
  #[texture(0)]
  #[sampler(1)]
  source_image: Handle<Image>,
  #[uniform(2)]
  time: f32,
}

impl Material2d for PostProcessingMaterial {
  fn fragment_shader() -> ShaderRef {
      "shaders/post.wgsl".into()
  }
}

fn update_post(
  mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
  image: Res<RenderImage>,
  time: Res<Time>,
) {
  // update the time uniform in the shader
  let mat = post_processing_materials.get_mut(&image.material);
  if let Some(mat) = mat {
    mat.time = time.elapsed_seconds();
  }

  println!("frame {}ms ({:.2}fps)", time.delta_seconds() * 1000.0, time.delta_seconds().recip())
}
