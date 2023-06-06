//===============================================================

use cfg_if::cfg_if;
use shipyard::{IntoWorkload, Workload, World};

//===============================================================

pub mod components;
pub(crate) mod systems;
pub mod tools;

cfg_if! {
    if #[cfg(feature = "2d")] {
        pub mod components_2d;
        pub(crate) mod systems_2d;
        pub mod tools_2d;
    }
}

cfg_if! {
    if #[cfg(feature = "3d")] {
        pub mod components_3d;
        pub(crate) mod systems_3d;
        pub mod tools_3d;
    }
}

//===============================================================

pub(crate) fn run_startup_systems(world: &mut World) {
    #[cfg(feature = "2d")]
    world.run(systems_2d::sys_setup_texture_renderer);
}

pub(crate) fn run_resize_systems(world: &mut World) {
    world.run(systems::sys_resize_camera);

    cfg_if! {
    if #[cfg(feature = "2d")] {
            world.run(systems_2d::sys_resize_pipeline);
        }
    }

    #[cfg(feature = "3d")]
    world.run(systems_3d::sys_resize_pipeline);
}

pub(crate) fn workload_post_update_systems() -> Workload {
    // #[cfg(feature = "2d")]
    (systems_2d::sys_update_camera).into_workload()
}

//===============================================================

pub(crate) fn run_post_render_systems(world: &mut World) {
    cfg_if! {
        if #[cfg(feature = "2d")] {


            //--------------------------------------------------
            #[cfg(feature = "debug")]
            let instant = std::time::Instant::now();

            world.run(systems_2d::sys_process_textures);

            #[cfg(feature = "debug")]
            world
                .run(|mut debug_log: shipyard::UniqueViewMut<crate::core_components::TimingsDebug>| {
                    debug_log.add_log("Process Textures  total".into(), instant.elapsed().as_secs_f32());
                });
            //--------------------------------------------------
            #[cfg(feature = "debug")]
            let instant = std::time::Instant::now();

            world.run(systems_2d::sys_add_new_textures);

            #[cfg(feature = "debug")]
            world
                .run(|mut debug_log: shipyard::UniqueViewMut<crate::core_components::TimingsDebug>| {
                    debug_log.add_log("Add new textures total".into(), instant.elapsed().as_secs_f32());
                });
            //--------------------------------------------------
            #[cfg(feature = "debug")]
            let instant = std::time::Instant::now();

            world.run(systems_2d::sys_remove_unloaded_textures);

            #[cfg(feature = "debug")]
            world
                .run(|mut debug_log: shipyard::UniqueViewMut<crate::core_components::TimingsDebug>| {
                    debug_log.add_log("remove unloaded textures total".into(), instant.elapsed().as_secs_f32());
                });
            //--------------------------------------------------
            #[cfg(feature = "debug")]
            let instant = std::time::Instant::now();

            world.run(systems_2d::sys_render_textures);

            #[cfg(feature = "debug")]
            world
                .run(|mut debug_log: shipyard::UniqueViewMut<crate::core_components::TimingsDebug>| {
                    debug_log.add_log("render textures total".into(), instant.elapsed().as_secs_f32());
                });
            //--------------------------------------------------



            // world.run(systems_2d::sys_process_textures);
            // world.run(systems_2d::sys_add_new_textures);
            // world.run(systems_2d::sys_remove_unloaded_textures);
            // world.run(systems_2d::sys_render_textures);

        }
    }
    cfg_if! {
        if #[cfg(feature = "3d")] {
            world.run(systems_3d::sys_process_models);
            world.run(systems_3d::sys_render_models);
        }
    }

    systems::sys_end_render_pass(world);
}

//===============================================================
