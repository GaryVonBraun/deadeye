use bevy::prelude::*;

use crate::{
    core::states::AppState,
    map::messages::DeleteMapMessage,
    mission::editor::messages::{CreateMissionMessage, DeleteMissionMessage, EditMissionMessage},
    ui::missions_menu::components::{MissionListInteractions, MissionMenuInteractions},
};

pub fn mission_menu_interactions(
    mut button_query: Query<
        (&Interaction, &MissionMenuInteractions),
        (Changed<Interaction>, With<MissionMenuInteractions>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut create_mission_message_writer: MessageWriter<CreateMissionMessage>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                MissionMenuInteractions::Back => {
                    next_state.set(AppState::MainMenu);
                }
                MissionMenuInteractions::New => {
                    create_mission_message_writer.write(CreateMissionMessage);
                }
            }
        }
    }
}
pub fn mission_list_interactions(
    mut button_query: Query<
        (&Interaction, &MissionListInteractions),
        (Changed<Interaction>, With<MissionListInteractions>),
    >,
    mut delete_mission_writer: MessageWriter<DeleteMissionMessage>,
    mut delete_map_writer: MessageWriter<DeleteMapMessage>,
    mut edit_mission_writer: MessageWriter<EditMissionMessage>,
) {
    for (interaction, &menu_interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match menu_interaction {
                MissionListInteractions::Delete { mission_id, map_id } => {
                    delete_mission_writer.write(DeleteMissionMessage { id: mission_id });
                    delete_map_writer.write(DeleteMapMessage { id: map_id });
                }
                MissionListInteractions::Edit(id) => {
                    edit_mission_writer.write(EditMissionMessage { id });
                }
            }
        }
    }
}
