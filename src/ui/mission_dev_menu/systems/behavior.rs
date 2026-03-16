use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::mission_dev_menu::components::{MissionDevListInteractions, MissionDevMenuInteractions},
    world::map::messages::{CreateMissionMessage, DeleteMissionMessage, EditMissionMessage},
};

pub fn mission_menu_interactions(
    mut button_query: Query<
        (&Interaction, &MissionDevMenuInteractions),
        (Changed<Interaction>, With<MissionDevMenuInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut create_mission_message_writer: MessageWriter<CreateMissionMessage>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                MissionDevMenuInteractions::Back => {
                    next_state.set(AppState::MainMenu);
                }
                MissionDevMenuInteractions::New => {
                    create_mission_message_writer.write(CreateMissionMessage);
                }
            }
        }
    }
}
pub fn mission_list_interactions(
    mut button_query: Query<
        (&Interaction, &MissionDevListInteractions),
        (Changed<Interaction>, With<MissionDevListInteractions>),
    >,
    mut delete_mission_writer: MessageWriter<DeleteMissionMessage>,
    mut edit_mission_writer: MessageWriter<EditMissionMessage>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                MissionDevListInteractions::Delete(id) => {
                    delete_mission_writer.write(DeleteMissionMessage { id });
                }
                MissionDevListInteractions::Edit(id) => {
                    edit_mission_writer.write(EditMissionMessage { id });
                }
            }
        }
    }
}
