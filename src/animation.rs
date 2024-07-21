
// use bevy::{
//     animation::animate_targets, asset::AssetMetaCheck::Never, prelude::*
// };

// #[derive(Resource)]
// struct Animations {
//     animations: Vec<AnimationNodeIndex>,
//     graph: Handle<AnimationGraph>
// }

// Once the scene is loaded, start the animation
// fn setup_animation_once_loaded(
//     mut commands: Commands,
//     animations: Res<Animations>,
//     mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
// ) {
//     for (entity, mut player) in &mut players {
//         let mut transitions = AnimationTransitions::new();

//         // Make sure to start the animation via the `AnimationTransitions`
//         // component. The `AnimationTransitions` component wants to manage all
//         // the animations and will get confused if the animations are started
//         // directly via the `AnimationPlayer`.
//         transitions
//             .play(&mut player, animations.animations[0], Duration::ZERO)
//             .repeat();

//         commands
//             .entity(entity)
//             .insert(animations.graph.clone())
//             .insert(transitions);
//     }
// }


// fn keyboard_animation_control(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
//     animations: Res<Animations>,
//     mut current_animation: Local<usize>,
// ) {
//     for (mut player, mut transitions) in &mut animation_players {
//         if keyboard_input.just_pressed(KeyCode::Enter) {
//             *current_animation = (*current_animation + 1) % animations.animations.len();

//             transitions
//                 .play(
//                     &mut player,
//                     animations.animations[*current_animation],
//                     Duration::from_secs(10)
//                 )
//                 .repeat();

//         }
//     }
// }