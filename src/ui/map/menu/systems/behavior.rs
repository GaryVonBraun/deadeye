use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::map::menu::components::{MapListInteractions, MapMenuInteractions},
    world::map::messages::{CreateMapMessage, DeleteMapMessage},
};

pub fn map_menu_interactions(
    mut button_query: Query<
        (&Interaction, &MapMenuInteractions),
        (Changed<Interaction>, With<MapMenuInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut create_map_message_writer: MessageWriter<CreateMapMessage>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                MapMenuInteractions::Back => {
                    next_state.set(AppState::MainMenu);
                }
                MapMenuInteractions::New => {
                    create_map_message_writer.write(CreateMapMessage);
                }
            }
        }
    }
}
pub fn map_list_interactions(
    mut button_query: Query<
        (&Interaction, &MapListInteractions),
        (Changed<Interaction>, With<MapListInteractions>),
    >,
    mut delete_map_message_writer: MessageWriter<DeleteMapMessage>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                MapListInteractions::Delete(id) => {
                    delete_map_message_writer.write(DeleteMapMessage { id });
                }
                MapListInteractions::Edit => {
                    //TEMPORARY - currently settings don't exist so its placeholder
                    todo!()
                }
            }
        }
    }
}
