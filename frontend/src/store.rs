use common::{Engagement, SelectOption};
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::components::consults::consults_display::ResponseConsult;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
    pub alert_typ: i32,
}

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub user_id: Option<i32>,
    pub token: Option<String>,
    pub is_authenticated: bool,
}

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ConsultStore {
    pub consults: Vec<ResponseConsult>,
}

pub fn set_consults(consults: Vec<ResponseConsult>, dispatch: Dispatch<ConsultStore>) {
    dispatch.reduce_mut(move |store| {
        store.consults = consults;
    })
}

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct OptionsStore {
    pub location_options: Option<Vec<SelectOption>>,
    pub consultant_options: Option<Vec<SelectOption>>
}

pub fn set_location_options(options: Vec<SelectOption>, dispatch: Dispatch<OptionsStore>) {
    dispatch.reduce_mut(move |store| {
        store.location_options = Some(options);
    })
}

pub fn set_username(username: String, dispatch: Dispatch<AuthStore>) {
    dispatch.reduce_mut(move |store| {
        store.username = Some(username);
    })
}

pub fn set_password(password: String, dispatch: Dispatch<AuthStore>) {
    dispatch.reduce_mut(move |store| {
        store.password = Some(password);
    })
}

pub fn set_token(token: String, dispatch: Dispatch<AuthStore>) {
    dispatch.reduce_mut(move |store| {
        store.token = Some(token);
    })
}

pub fn set_user_id(user_id: i32, dispatch: Dispatch<AuthStore>) {
    dispatch.reduce_mut(move |store| {
        store.user_id = Some(user_id);
    })
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

pub fn set_show_alert(message: String, typ: i32, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            alert_typ: typ,
            show_alert: true,
        };
    })
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}
