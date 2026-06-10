use uuid::Uuid;

use crate::campaign::{
    io::operations::{list_all_campaign_data, write_campaign},
    resources::Campaign,
};

pub fn create_new_campaign() {
    let campaign = Campaign {
        id: Uuid::new_v4(),
        name: "new save".to_string(),
        money: 500,
    };

    write_campaign(&campaign);
}

pub fn load_campaigns() -> Vec<Campaign> {
    list_all_campaign_data()
}
