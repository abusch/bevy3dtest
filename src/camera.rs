use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        dof::{DepthOfFieldMode, DepthOfFieldSettings},
    },
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters},
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

pub(crate) fn plugin(app: &mut App) {
    // Spawn the main camera.
    app.add_plugins((
        LookTransformPlugin,
        ResourceInspectorPlugin::<CameraParameters>::default(),
    ))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, update_camera)
    .register_type::<CameraParameters>()
    .insert_resource(CameraParameters {
        aperture_f_stops: 2.8,
        shutter_speed_s: 0.02,
        sensitivity_iso: 100.0,
        sensor_height: 0.016, // for width = 35mm
    });
}

// PhysicalCameraParameters doesn't implement `Reflect` for some reason...
#[derive(Default, Copy, Clone, Resource, Reflect)]
pub struct CameraParameters {
    aperture_f_stops: f32,
    shutter_speed_s: f32,
    sensitivity_iso: f32,
    sensor_height: f32,
}

impl From<CameraParameters> for PhysicalCameraParameters {
    fn from(
        CameraParameters {
            aperture_f_stops,
            shutter_speed_s,
            sensitivity_iso,
            sensor_height,
        }: CameraParameters,
    ) -> Self {
        PhysicalCameraParameters {
            aperture_f_stops,
            shutter_speed_s,
            sensitivity_iso,
            sensor_height,
        }
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    // Camera
    commands.spawn((
        Name::new("3D Camera"),
        MainCamera,
        LookTransformBundle {
            transform: LookTransform::default(),
            smoother: Smoother::new(0.9),
        },
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings::NATURAL,
        DepthOfFieldSettings {
            mode: DepthOfFieldMode::Bokeh,
            ..default()
        },
        IsDefaultUiCamera,
    ));
}

fn update_camera(
    mut exposure: Query<(&mut Exposure, &mut DepthOfFieldSettings)>,
    params: Res<CameraParameters>,
) {
    let (mut exposure, mut dof) = exposure.single_mut();
    let physical_params: PhysicalCameraParameters = (*params).into();
    *exposure = Exposure::from_physical_camera(physical_params);
    *dof = DepthOfFieldSettings {
        mode: DepthOfFieldMode::Bokeh,
        focal_distance: 15.0,
        ..DepthOfFieldSettings::from_physical_camera(&physical_params)
    };
}
