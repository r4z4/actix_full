use common::Engagement;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub engagements: Vec<Engagement>,
    pub loading: bool,
    pub alert_input: AlertInput,
}

pub fn set_engagement(engagement: Engagement, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.engagements.insert(0, engagement);
    })
}

pub fn set_engagement_list(engagements: Vec<Engagement>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.engagements = engagements;
    })
}

pub fn delete_engagement(id: uuid::Uuid, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.engagements.retain(|f| f.id != id);
    })
}

pub fn set_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.loading = loading;
    })
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
        };
    })
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}
