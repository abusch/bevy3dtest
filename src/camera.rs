use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        dof::{DepthOfFieldMode, DepthOfFieldSettings},
    },
    math::vec3,
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters},
};
use bevy_dolly::{
    prelude::{Arm, LookAt, Position, Rig, Rotation, Smooth},
    system::Dolly,
};

pub(crate) fn plugin(app: &mut App) {
    // Spawn the main camera.
    app.add_systems(Startup, spawn_camera)
        .add_systems(Update, (Dolly::<MainCamera>::update_active, update_camera))
        .register_type::<CameraParameters>()
        .insert_resource(CameraParameters {
            aperture_f_stops: 2.8,
            shutter_speed_s: 0.02,
            sensitivity_iso: 100.0,
            sensor_height: 0.016, // for width = 35mm
        });
    #[cfg(feature = "dev")]
    app.add_plugins((bevy_inspector_egui::quick::ResourceInspectorPlugin::<
        CameraParameters,
    >::default(),));
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
    let start_pos = vec3(0.0, 1.0, 7.0);
    commands.spawn((
        Name::new("3D Camera"),
        MainCamera,
        Rig::builder()
            .with(Position::new(start_pos))
            .with(Rotation::new(Quat::IDENTITY))
            .with(Smooth::new_position(1.25).predictive(true))
            .with(Arm::new(Vec3::new(0.0, 1.0, 5.0)))
            .with(Smooth::new_position(2.5))
            .with(
                LookAt::new(start_pos)
                    .tracking_smoothness(1.25)
                    .tracking_predictive(true),
            )
            .build(),
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
