use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::map::menu::{
        components::{MapListInteractions, MapMenuInteractions},
        messages::RefreshMapListMessage,
    },
    world::map::messages::DeleteMapMessage,
};

pub fn map_menu_interactions(
    mut button_query: Query<
        (&Interaction, &MapMenuInteractions),
        (Changed<Interaction>, With<MapMenuInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                MapMenuInteractions::Back => {
                    next_state.set(AppState::MainMenu);
                }
                MapMenuInteractions::New => {
                    //TEMPORARY - currently settings don't exist so its placeholder
                    todo!()
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
    mut resfresh_map_message_writer: MessageWriter<RefreshMapListMessage>,
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
