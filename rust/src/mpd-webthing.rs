extern crate mpd;
extern crate rand;
#[macro_use]
extern crate serde_json;
extern crate uuid;
extern crate webthing;

use mpd::client::Client;
use mpd::error::Error;
use mpd::search::{Query, Term};
use mpd::song::Song;
use mpd::status::{State, Status};
use rand::Rng;
use std::any::Any;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};
use std::vec::Drain;
use std::{thread, time};
use uuid::Uuid;
use webthing::action::{Action, BaseAction};
use webthing::event::{BaseEvent, Event};
use webthing::property::{BaseProperty, Property, ValueForwarder};
use webthing::server::{ActionGenerator, ThingsType, WebThingServer};
use webthing::thing::{BaseThing, Thing};

struct PlayAction(BaseAction);

impl PlayAction {
    fn new(
        input: Option<serde_json::Map<String, serde_json::Value>>,
        thing: Weak<RwLock<Box<Thing>>>,
    ) -> PlayAction {
        PlayAction(BaseAction::new(
            Uuid::new_v4().to_string(),
            "play".to_owned(),
            input,
            thing,
        ))
    }
}

impl Action for PlayAction {
    fn set_href_prefix(&mut self, prefix: String) {
        self.0.set_href_prefix(prefix)
    }

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_href(&self) -> String {
        self.0.get_href()
    }

    fn get_status(&self) -> String {
        self.0.get_status()
    }

    fn get_time_requested(&self) -> String {
        self.0.get_time_requested()
    }

    fn get_time_completed(&self) -> Option<String> {
        self.0.get_time_completed()
    }

    fn get_input(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.0.get_input()
    }

    fn get_thing(&self) -> Option<Arc<RwLock<Box<Thing>>>> {
        self.0.get_thing()
    }

    fn set_status(&mut self, status: String) {
        self.0.set_status(status)
    }

    fn start(&mut self) {
        self.0.start()
    }

    fn perform_action(&mut self) {
        let thing = self.get_thing();
        if thing.is_none() {
            return;
        }

        let thing = thing.unwrap();
        let name = self.get_name();
        let id = self.get_id();

        thread::spawn(move || {
            let thing = thing.clone();
            let mut thing = thing.write().unwrap();
            let thing = thing.as_mut_any().downcast_mut::<MPDThing>().unwrap();
            let client = thing.get_client();
            let mut client = client.write().unwrap();

            let _ = MPDThing::play(&mut *client);

            thing.finish_action(name, id);
        });
    }

    fn cancel(&mut self) {
        self.0.cancel()
    }

    fn finish(&mut self) {
        self.0.finish()
    }
}

struct PauseAction(BaseAction);

impl PauseAction {
    fn new(
        input: Option<serde_json::Map<String, serde_json::Value>>,
        thing: Weak<RwLock<Box<Thing>>>,
    ) -> PauseAction {
        PauseAction(BaseAction::new(
            Uuid::new_v4().to_string(),
            "pause".to_owned(),
            input,
            thing,
        ))
    }
}

impl Action for PauseAction {
    fn set_href_prefix(&mut self, prefix: String) {
        self.0.set_href_prefix(prefix)
    }

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_href(&self) -> String {
        self.0.get_href()
    }

    fn get_status(&self) -> String {
        self.0.get_status()
    }

    fn get_time_requested(&self) -> String {
        self.0.get_time_requested()
    }

    fn get_time_completed(&self) -> Option<String> {
        self.0.get_time_completed()
    }

    fn get_input(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.0.get_input()
    }

    fn get_thing(&self) -> Option<Arc<RwLock<Box<Thing>>>> {
        self.0.get_thing()
    }

    fn set_status(&mut self, status: String) {
        self.0.set_status(status)
    }

    fn start(&mut self) {
        self.0.start()
    }

    fn perform_action(&mut self) {
        let thing = self.get_thing();
        if thing.is_none() {
            return;
        }

        let thing = thing.unwrap();
        let name = self.get_name();
        let id = self.get_id();

        thread::spawn(move || {
            let thing = thing.clone();
            let mut thing = thing.write().unwrap();
            let thing = thing.as_mut_any().downcast_mut::<MPDThing>().unwrap();
            let client = thing.get_client();
            let mut client = client.write().unwrap();

            let _ = MPDThing::pause(&mut *client);

            thing.finish_action(name, id);
        });
    }

    fn cancel(&mut self) {
        self.0.cancel()
    }

    fn finish(&mut self) {
        self.0.finish()
    }
}

struct StopAction(BaseAction);

impl StopAction {
    fn new(
        input: Option<serde_json::Map<String, serde_json::Value>>,
        thing: Weak<RwLock<Box<Thing>>>,
    ) -> StopAction {
        StopAction(BaseAction::new(
            Uuid::new_v4().to_string(),
            "stop".to_owned(),
            input,
            thing,
        ))
    }
}

impl Action for StopAction {
    fn set_href_prefix(&mut self, prefix: String) {
        self.0.set_href_prefix(prefix)
    }

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_href(&self) -> String {
        self.0.get_href()
    }

    fn get_status(&self) -> String {
        self.0.get_status()
    }

    fn get_time_requested(&self) -> String {
        self.0.get_time_requested()
    }

    fn get_time_completed(&self) -> Option<String> {
        self.0.get_time_completed()
    }

    fn get_input(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.0.get_input()
    }

    fn get_thing(&self) -> Option<Arc<RwLock<Box<Thing>>>> {
        self.0.get_thing()
    }

    fn set_status(&mut self, status: String) {
        self.0.set_status(status)
    }

    fn start(&mut self) {
        self.0.start()
    }

    fn perform_action(&mut self) {
        let thing = self.get_thing();
        if thing.is_none() {
            return;
        }

        let thing = thing.unwrap();
        let name = self.get_name();
        let id = self.get_id();

        thread::spawn(move || {
            let thing = thing.clone();
            let mut thing = thing.write().unwrap();
            let thing = thing.as_mut_any().downcast_mut::<MPDThing>().unwrap();
            let client = thing.get_client();
            let mut client = client.write().unwrap();

            let _ = MPDThing::stop(&mut *client);

            thing.finish_action(name, id);
        });
    }

    fn cancel(&mut self) {
        self.0.cancel()
    }

    fn finish(&mut self) {
        self.0.finish()
    }
}

struct NextAction(BaseAction);

impl NextAction {
    fn new(
        input: Option<serde_json::Map<String, serde_json::Value>>,
        thing: Weak<RwLock<Box<Thing>>>,
    ) -> NextAction {
        NextAction(BaseAction::new(
            Uuid::new_v4().to_string(),
            "next".to_owned(),
            input,
            thing,
        ))
    }
}

impl Action for NextAction {
    fn set_href_prefix(&mut self, prefix: String) {
        self.0.set_href_prefix(prefix)
    }

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_href(&self) -> String {
        self.0.get_href()
    }

    fn get_status(&self) -> String {
        self.0.get_status()
    }

    fn get_time_requested(&self) -> String {
        self.0.get_time_requested()
    }

    fn get_time_completed(&self) -> Option<String> {
        self.0.get_time_completed()
    }

    fn get_input(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.0.get_input()
    }

    fn get_thing(&self) -> Option<Arc<RwLock<Box<Thing>>>> {
        self.0.get_thing()
    }

    fn set_status(&mut self, status: String) {
        self.0.set_status(status)
    }

    fn start(&mut self) {
        self.0.start()
    }

    fn perform_action(&mut self) {
        let thing = self.get_thing();
        if thing.is_none() {
            return;
        }

        let thing = thing.unwrap();
        let name = self.get_name();
        let id = self.get_id();

        thread::spawn(move || {
            let thing = thing.clone();
            let mut thing = thing.write().unwrap();
            let thing = thing.as_mut_any().downcast_mut::<MPDThing>().unwrap();
            let client = thing.get_client();
            let mut client = client.write().unwrap();

            let _ = MPDThing::next(&mut *client);

            thing.finish_action(name, id);
        });
    }

    fn cancel(&mut self) {
        self.0.cancel()
    }

    fn finish(&mut self) {
        self.0.finish()
    }
}

struct PreviousAction(BaseAction);

impl PreviousAction {
    fn new(
        input: Option<serde_json::Map<String, serde_json::Value>>,
        thing: Weak<RwLock<Box<Thing>>>,
    ) -> PreviousAction {
        PreviousAction(BaseAction::new(
            Uuid::new_v4().to_string(),
            "previous".to_owned(),
            input,
            thing,
        ))
    }
}

impl Action for PreviousAction {
    fn set_href_prefix(&mut self, prefix: String) {
        self.0.set_href_prefix(prefix)
    }

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_href(&self) -> String {
        self.0.get_href()
    }

    fn get_status(&self) -> String {
        self.0.get_status()
    }

    fn get_time_requested(&self) -> String {
        self.0.get_time_requested()
    }

    fn get_time_completed(&self) -> Option<String> {
        self.0.get_time_completed()
    }

    fn get_input(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.0.get_input()
    }

    fn get_thing(&self) -> Option<Arc<RwLock<Box<Thing>>>> {
        self.0.get_thing()
    }

    fn set_status(&mut self, status: String) {
        self.0.set_status(status)
    }

    fn start(&mut self) {
        self.0.start()
    }

    fn perform_action(&mut self) {
        let thing = self.get_thing();
        if thing.is_none() {
            return;
        }

        let thing = thing.unwrap();
        let name = self.get_name();
        let id = self.get_id();

        thread::spawn(move || {
            let thing = thing.clone();
            let mut thing = thing.write().unwrap();
            let thing = thing.as_mut_any().downcast_mut::<MPDThing>().unwrap();
            let client = thing.get_client();
            let mut client = client.write().unwrap();

            let _ = MPDThing::previous(&mut *client);

            thing.finish_action(name, id);
        });
    }

    fn cancel(&mut self) {
        self.0.cancel()
    }

    fn finish(&mut self) {
        self.0.finish()
    }
}

struct QueueRandomAction(BaseAction);

impl QueueRandomAction {
    fn new(
        input: Option<serde_json::Map<String, serde_json::Value>>,
        thing: Weak<RwLock<Box<Thing>>>,
    ) -> QueueRandomAction {
        QueueRandomAction(BaseAction::new(
            Uuid::new_v4().to_string(),
            "queueRandom".to_owned(),
            input,
            thing,
        ))
    }
}

impl Action for QueueRandomAction {
    fn set_href_prefix(&mut self, prefix: String) {
        self.0.set_href_prefix(prefix)
    }

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_href(&self) -> String {
        self.0.get_href()
    }

    fn get_status(&self) -> String {
        self.0.get_status()
    }

    fn get_time_requested(&self) -> String {
        self.0.get_time_requested()
    }

    fn get_time_completed(&self) -> Option<String> {
        self.0.get_time_completed()
    }

    fn get_input(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.0.get_input()
    }

    fn get_thing(&self) -> Option<Arc<RwLock<Box<Thing>>>> {
        self.0.get_thing()
    }

    fn set_status(&mut self, status: String) {
        self.0.set_status(status)
    }

    fn start(&mut self) {
        self.0.start()
    }

    fn perform_action(&mut self) {
        let thing = self.get_thing();
        if thing.is_none() {
            return;
        }

        let thing = thing.unwrap();
        let input = self.get_input().unwrap().clone();
        let name = self.get_name();
        let id = self.get_id();

        thread::spawn(move || {
            let thing = thing.clone();
            let mut thing = thing.write().unwrap();
            let thing = thing.as_mut_any().downcast_mut::<MPDThing>().unwrap();
            let client = thing.get_client();
            let mut client = client.write().unwrap();

            let songs = MPDThing::list(&mut *client);
            if songs.is_ok() {
                let songs = songs.ok().unwrap();
                let count = input.get("count").unwrap().as_u64().unwrap();
                let mut rng = rand::thread_rng();

                for _ in 0..count {
                    let choice: usize = rng.gen_range(0, songs.len());
                    let _ = MPDThing::add(&mut *client, songs[choice].clone());
                }

                match MPDThing::get_playlist(&mut *client) {
                    Ok(playlist) => {
                        thing.add_event(Box::new(PlaylistUpdatedEvent::new(Some(json!(playlist)))))
                    }
                    Err(_) => (),
                }
            }

            thing.finish_action(name, id);
        });
    }

    fn cancel(&mut self) {
        self.0.cancel()
    }

    fn finish(&mut self) {
        self.0.finish()
    }
}

struct VolumeForwarder(Weak<RwLock<Client>>);

impl ValueForwarder for VolumeForwarder {
    fn set_value(&mut self, value: serde_json::Value) -> Result<serde_json::Value, &'static str> {
        if !value.is_i64() {
            return Err("Invalid value");
        }

        let volume: i8 = value.as_i64().unwrap() as i8;
        match self.0.upgrade() {
            Some(client) => match client.write().unwrap().volume(volume) {
                Ok(_) => Ok(value),
                Err(_) => Err("Failed to set value"),
            },
            None => Err("Client reference disappeared"),
        }
    }
}

struct RepeatForwarder(Weak<RwLock<Client>>);

impl ValueForwarder for RepeatForwarder {
    fn set_value(&mut self, value: serde_json::Value) -> Result<serde_json::Value, &'static str> {
        if !value.is_boolean() {
            return Err("Invalid value");
        }

        let repeat = value.as_bool().unwrap();
        match self.0.upgrade() {
            Some(client) => match client.write().unwrap().repeat(repeat) {
                Ok(_) => Ok(value),
                Err(_) => Err("Failed to set value"),
            },
            None => Err("Client reference disappeared"),
        }
    }
}

struct RandomForwarder(Weak<RwLock<Client>>);

impl ValueForwarder for RandomForwarder {
    fn set_value(&mut self, value: serde_json::Value) -> Result<serde_json::Value, &'static str> {
        if !value.is_boolean() {
            return Err("Invalid value");
        }

        let random = value.as_bool().unwrap();
        match self.0.upgrade() {
            Some(client) => match client.write().unwrap().random(random) {
                Ok(_) => Ok(value),
                Err(_) => Err("Failed to set value"),
            },
            None => Err("Client reference disappeared"),
        }
    }
}

pub struct PlaylistUpdatedEvent(BaseEvent);

impl PlaylistUpdatedEvent {
    fn new(data: Option<serde_json::Value>) -> PlaylistUpdatedEvent {
        PlaylistUpdatedEvent(BaseEvent::new("playlistUpdated".to_owned(), data))
    }
}

impl Event for PlaylistUpdatedEvent {
    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_data(&self) -> Option<serde_json::Value> {
        self.0.get_data()
    }

    fn get_time(&self) -> String {
        self.0.get_time()
    }
}

struct MPDThing {
    base: BaseThing,
    client: Arc<RwLock<Client>>,
}

impl MPDThing {
    fn new() -> MPDThing {
        let mut base = BaseThing::new(
            "MPD".to_owned(),
            Some(vec![]),
            Some("Music Player Daemon".to_owned()),
        );

        let client = Arc::new(RwLock::new(Client::connect("127.0.0.1:6600").unwrap()));

        let status;
        let song;
        let volume;
        let repeat;
        let random;
        let state;
        let artist;
        let album;
        let title;
        {
            let mut client = client.write().unwrap();

            status = client.status().unwrap();
            volume = MPDThing::get_volume(&mut *client, Some(&status));
            repeat = MPDThing::get_repeat(&mut *client, Some(&status));
            random = MPDThing::get_random(&mut *client, Some(&status));
            state = MPDThing::get_state(&mut *client, Some(&status));

            song = client.currentsong().unwrap();
            artist = MPDThing::get_artist(&mut *client, &song);
            album = MPDThing::get_album(&mut *client, &song);
            title = MPDThing::get_title(&mut *client, &song);
        }

        // Add a 'volume' property.
        let volume_description = json!({
            "type": "number",
            "description": "Playback volume",
            "minimum": 0,
            "maximum": 100,
            "label": "Volume",
        });
        let volume_description = volume_description.as_object().unwrap().clone();
        base.add_property(Box::new(BaseProperty::new(
            "volume".to_owned(),
            json!(volume),
            Some(Box::new(VolumeForwarder(Arc::downgrade(&client)))),
            Some(volume_description),
        )));

        // Add a 'repeat' property.
        let repeat_description = json!({
            "type": "boolean",
            "description": "Repeat mode",
            "label": "Repeat",
        });
        let repeat_description = repeat_description.as_object().unwrap().clone();
        base.add_property(Box::new(BaseProperty::new(
            "repeat".to_owned(),
            json!(repeat),
            Some(Box::new(RepeatForwarder(Arc::downgrade(&client)))),
            Some(repeat_description),
        )));

        // Add a 'random' property.
        let random_description = json!({
            "type": "boolean",
            "description": "Random mode",
            "label": "Random",
        });
        let random_description = random_description.as_object().unwrap().clone();
        base.add_property(Box::new(BaseProperty::new(
            "random".to_owned(),
            json!(random),
            Some(Box::new(RandomForwarder(Arc::downgrade(&client)))),
            Some(random_description),
        )));

        // Add a 'state' property, which indicates playback state.
        let state_description = json!({
            "type": "string",
            "description": "Current playback state",
            "label": "State",
        });
        let state_description = state_description.as_object().unwrap().clone();
        base.add_property(Box::new(BaseProperty::new(
            "state".to_owned(),
            json!(state),
            None,
            Some(state_description),
        )));

        // Add an 'artist' property.
        let artist_description = json!({
            "type": "string",
            "description": "Artist of current song",
            "label": "Artist",
        });
        let artist_description = artist_description.as_object().unwrap().clone();
        base.add_property(Box::new(BaseProperty::new(
            "artist".to_owned(),
            json!(artist),
            None,
            Some(artist_description),
        )));

        // Add an 'album' property.
        let album_description = json!({
            "type": "string",
            "description": "Album current song belongs to",
            "label": "Album",
        });
        let album_description = album_description.as_object().unwrap().clone();
        base.add_property(Box::new(BaseProperty::new(
            "album".to_owned(),
            json!(album),
            None,
            Some(album_description),
        )));

        // Add a 'title' property.
        let title_description = json!({
            "type": "string",
            "description": "Title of current song",
            "label": "Title",
        });
        let title_description = title_description.as_object().unwrap().clone();
        base.add_property(Box::new(BaseProperty::new(
            "title".to_owned(),
            json!(title),
            None,
            Some(title_description),
        )));

        // Add a 'play' action.
        let play_metadata = json!({
            "description": "Start playback",
            "label": "Play",
        });
        let play_metadata = play_metadata.as_object().unwrap().clone();
        base.add_available_action("play".to_owned(), play_metadata);

        // Add a 'pause' action.
        let pause_metadata = json!({
            "description": "Pause playback",
            "label": "Pause",
        });
        let pause_metadata = pause_metadata.as_object().unwrap().clone();
        base.add_available_action("pause".to_owned(), pause_metadata);

        // Add a 'stop' action.
        let stop_metadata = json!({
            "description": "Stop playback",
            "label": "Stop",
        });
        let stop_metadata = stop_metadata.as_object().unwrap().clone();
        base.add_available_action("stop".to_owned(), stop_metadata);

        // Add a 'next' action.
        let next_metadata = json!({
            "description": "Skip to next song",
            "label": "Next",
        });
        let next_metadata = next_metadata.as_object().unwrap().clone();
        base.add_available_action("next".to_owned(), next_metadata);

        // Add a 'previous' action.
        let previous_metadata = json!({
            "description": "Skip to previous song",
            "label": "Previous",
        });
        let previous_metadata = previous_metadata.as_object().unwrap().clone();
        base.add_available_action("previous".to_owned(), previous_metadata);

        // Add a 'queueRandom' action.
        let queue_random_metadata = json!({
            "description": "Queue a series of random songs",
            "label": "Queue Random",
            "input": {
                "type": "object",
                "required": [
                    "count",
                ],
                "properties": {
                    "count": {
                        "type": "number",
                        "minimum": 1,
                    },
                },
            },
        });
        let queue_random_metadata = queue_random_metadata.as_object().unwrap().clone();
        base.add_available_action("queueRandom".to_owned(), queue_random_metadata);

        // Add a 'playlistUpdated' event.
        let playlist_updated_metadata = json!({
            "description": "The current playlist has been updated",
            "type": "string",
        });
        let playlist_updated_metadata = playlist_updated_metadata.as_object().unwrap().clone();
        base.add_available_event("playlistUpdated".to_owned(), playlist_updated_metadata);

        MPDThing {
            base: base,
            client: client,
        }
    }

    fn get_client(&self) -> Arc<RwLock<Client>> {
        self.client.clone()
    }

    fn get_volume(client: &mut Client, status: Option<&Status>) -> i8 {
        match status {
            Some(status) => status.volume,
            None => client.status().unwrap().volume,
        }
    }

    fn get_repeat(client: &mut Client, status: Option<&Status>) -> bool {
        match status {
            Some(status) => status.repeat,
            None => client.status().unwrap().repeat,
        }
    }

    fn get_random(client: &mut Client, status: Option<&Status>) -> bool {
        match status {
            Some(status) => status.random,
            None => client.status().unwrap().random,
        }
    }

    fn get_state(client: &mut Client, status: Option<&Status>) -> &'static str {
        let state = match status {
            Some(status) => status.state,
            None => client.status().unwrap().state,
        };

        match state {
            State::Stop => "stop",
            State::Play => "play",
            State::Pause => "pause",
        }
    }

    fn get_artist(client: &mut Client, song: &Option<Song>) -> Option<String> {
        match song {
            Some(song) => match song.tags.get("Artist") {
                Some(artist) => Some(artist.clone()),
                None => None,
            },
            None => match client.currentsong().unwrap() {
                Some(song) => match song.tags.get("Artist") {
                    Some(artist) => Some(artist.clone()),
                    None => None,
                },
                None => None,
            },
        }
    }

    fn get_album(client: &mut Client, song: &Option<Song>) -> Option<String> {
        match song {
            Some(song) => match song.tags.get("Album") {
                Some(album) => Some(album.clone()),
                None => None,
            },
            None => match client.currentsong().unwrap() {
                Some(song) => match song.tags.get("Album") {
                    Some(album) => Some(album.clone()),
                    None => None,
                },
                None => None,
            },
        }
    }

    fn get_title(client: &mut Client, song: &Option<Song>) -> Option<String> {
        match song {
            Some(song) => song.title.clone(),
            None => match client.currentsong().unwrap() {
                Some(song) => song.title,
                None => None,
            },
        }
    }

    fn update_volume(&mut self, volume: Option<i8>) {
        let volume = json!(match volume {
            Some(v) => v,
            None => {
                let client = self.get_client();
                let mut client = client.write().unwrap();
                MPDThing::get_volume(&mut *client, None)
            }
        });

        let updated = {
            let prop = self.find_property("volume".to_owned()).unwrap();
            if prop.get_value() != volume {
                let _ = prop.set_cached_value(volume.clone());
                true
            } else {
                false
            }
        };

        if updated {
            self.property_notify("volume".to_owned(), volume);
        }
    }

    fn update_random(&mut self, random: Option<bool>) {
        let random = json!(match random {
            Some(v) => v,
            None => {
                let client = self.get_client();
                let mut client = client.write().unwrap();
                MPDThing::get_random(&mut *client, None)
            }
        });

        let updated = {
            let prop = self.find_property("random".to_owned()).unwrap();
            if prop.get_value() != random {
                let _ = prop.set_cached_value(random.clone());
                true
            } else {
                false
            }
        };

        if updated {
            self.property_notify("random".to_owned(), random);
        }
    }

    fn update_repeat(&mut self, repeat: Option<bool>) {
        let repeat = json!(match repeat {
            Some(v) => v,
            None => {
                let client = self.get_client();
                let mut client = client.write().unwrap();
                MPDThing::get_repeat(&mut *client, None)
            }
        });

        let updated = {
            let prop = self.find_property("repeat".to_owned()).unwrap();
            if prop.get_value() != repeat {
                let _ = prop.set_cached_value(repeat.clone());
                true
            } else {
                false
            }
        };

        if updated {
            self.property_notify("repeat".to_owned(), repeat);
        }
    }

    fn update_state(&mut self, state: Option<&'static str>) {
        let state = json!(match state {
            Some(v) => v,
            None => {
                let client = self.get_client();
                let mut client = client.write().unwrap();
                MPDThing::get_state(&mut *client, None)
            }
        });

        let updated = {
            let prop = self.find_property("state".to_owned()).unwrap();
            if prop.get_value() != state {
                let _ = prop.set_cached_value(state.clone());
                true
            } else {
                false
            }
        };

        if updated {
            self.property_notify("state".to_owned(), state);
        }
    }

    fn update_artist(&mut self, artist: Option<String>) {
        let artist = json!(match artist {
            Some(v) => Some(v),
            None => {
                let client = self.get_client();
                let mut client = client.write().unwrap();
                MPDThing::get_artist(&mut *client, &None)
            }
        });

        let updated = {
            let prop = self.find_property("artist".to_owned()).unwrap();
            if prop.get_value() != artist {
                let _ = prop.set_cached_value(artist.clone());
                true
            } else {
                false
            }
        };

        if updated {
            self.property_notify("artist".to_owned(), artist);
        }
    }

    fn update_album(&mut self, album: Option<String>) {
        let album = json!(match album {
            Some(v) => Some(v),
            None => {
                let client = self.get_client();
                let mut client = client.write().unwrap();
                MPDThing::get_album(&mut *client, &None)
            }
        });

        let updated = {
            let prop = self.find_property("album".to_owned()).unwrap();
            if prop.get_value() != album {
                let _ = prop.set_cached_value(album.clone());
                true
            } else {
                false
            }
        };

        if updated {
            self.property_notify("album".to_owned(), album);
        }
    }

    fn update_title(&mut self, title: Option<String>) {
        let title = json!(match title {
            Some(v) => Some(v),
            None => {
                let client = self.get_client();
                let mut client = client.write().unwrap();
                MPDThing::get_title(&mut *client, &None)
            }
        });

        let updated = {
            let prop = self.find_property("title".to_owned()).unwrap();
            if prop.get_value() != title {
                let _ = prop.set_cached_value(title.clone());
                true
            } else {
                false
            }
        };

        if updated {
            self.property_notify("title".to_owned(), title);
        }
    }

    fn play(client: &mut Client) -> Result<(), Error> {
        match client.status().unwrap().state {
            State::Stop => client.play(),
            State::Play => Ok(()),
            State::Pause => client.pause(false),
        }
    }

    fn pause(client: &mut Client) -> Result<(), Error> {
        match client.status().unwrap().state {
            State::Play => client.pause(true),
            State::Stop | State::Pause => Ok(()),
        }
    }

    fn stop(client: &mut Client) -> Result<(), Error> {
        match client.status().unwrap().state {
            State::Stop => Ok(()),
            State::Play | State::Pause => client.stop(),
        }
    }

    fn next(client: &mut Client) -> Result<(), Error> {
        match client.status().unwrap().state {
            State::Stop => Ok(()),
            State::Play | State::Pause => client.next(),
        }
    }

    fn previous(client: &mut Client) -> Result<(), Error> {
        match client.status().unwrap().state {
            State::Stop => Ok(()),
            State::Play | State::Pause => client.prev(),
        }
    }

    fn list(client: &mut Client) -> Result<Vec<String>, Error> {
        client.list(&Term::File, &Query::new())
    }

    fn add(client: &mut Client, uri: String) -> Result<(), Error> {
        let song = Song {
            file: uri,
            name: None,
            title: None,
            last_mod: None,
            duration: None,
            place: None,
            range: None,
            tags: BTreeMap::new(),
        };

        match client.push(song) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn get_playlist(client: &mut Client) -> Result<String, Error> {
        match client.queue() {
            Ok(songs) => Ok(songs
                .iter()
                .map(|ref song| {
                    let artist = match song.tags.get("Artist") {
                        Some(artist) => artist.clone(),
                        None => "Unknown".to_owned(),
                    };

                    let title = match song.title {
                        Some(ref title) => title.clone(),
                        None => "Unknown".to_owned(),
                    };

                    format!("{} - {}", artist, title)
                })
                .collect::<Vec<String>>()
                .join("\n")),
            Err(e) => Err(e),
        }
    }
}

impl Thing for MPDThing {
    fn as_thing_description(&self) -> serde_json::Map<String, serde_json::Value> {
        self.base.as_thing_description()
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut Any {
        self
    }

    fn get_href(&self) -> String {
        self.base.get_href()
    }

    fn get_href_prefix(&self) -> String {
        self.base.get_href_prefix()
    }

    fn get_ws_href(&self) -> Option<String> {
        self.base.get_ws_href()
    }

    fn get_ui_href(&self) -> Option<String> {
        self.base.get_ui_href()
    }

    fn set_href_prefix(&mut self, prefix: String) {
        self.base.set_href_prefix(prefix)
    }

    fn set_ws_href(&mut self, href: String) {
        self.base.set_ws_href(href)
    }

    fn set_ui_href(&mut self, href: String) {
        self.base.set_ui_href(href)
    }

    fn get_name(&self) -> String {
        self.base.get_name()
    }

    fn get_context(&self) -> String {
        self.base.get_context()
    }

    fn get_type(&self) -> Vec<String> {
        self.base.get_type()
    }

    fn get_description(&self) -> String {
        self.base.get_description()
    }

    fn get_property_descriptions(&self) -> serde_json::Map<String, serde_json::Value> {
        self.base.get_property_descriptions()
    }

    fn get_action_descriptions(&self, action_name: Option<String>) -> serde_json::Value {
        self.base.get_action_descriptions(action_name)
    }

    fn get_event_descriptions(&self, event_name: Option<String>) -> serde_json::Value {
        self.base.get_event_descriptions(event_name)
    }

    fn add_property(&mut self, property: Box<Property>) {
        self.base.add_property(property)
    }

    fn remove_property(&mut self, property_name: String) {
        self.base.remove_property(property_name)
    }

    fn find_property(&mut self, property_name: String) -> Option<&mut Box<Property>> {
        self.base.find_property(property_name)
    }

    fn get_property(&self, property_name: String) -> Option<serde_json::Value> {
        self.base.get_property(property_name)
    }

    fn get_properties(&self) -> serde_json::Map<String, serde_json::Value> {
        self.base.get_properties()
    }

    fn has_property(&self, property_name: String) -> bool {
        self.base.has_property(property_name)
    }

    fn get_action(
        &self,
        action_name: String,
        action_id: String,
    ) -> Option<Arc<RwLock<Box<Action>>>> {
        self.base.get_action(action_name, action_id)
    }

    fn add_event(&mut self, event: Box<Event>) {
        self.base.add_event(event)
    }

    fn add_available_event(
        &mut self,
        name: String,
        metadata: serde_json::Map<String, serde_json::Value>,
    ) {
        self.base.add_available_event(name, metadata)
    }

    fn add_action(
        &mut self,
        action: Arc<RwLock<Box<Action>>>,
        input: Option<&serde_json::Value>,
    ) -> Result<(), &str> {
        self.base.add_action(action, input)
    }

    fn remove_action(&mut self, action_name: String, action_id: String) -> bool {
        self.base.remove_action(action_name, action_id)
    }

    fn add_available_action(
        &mut self,
        name: String,
        metadata: serde_json::Map<String, serde_json::Value>,
    ) {
        self.base.add_available_action(name, metadata)
    }

    fn add_subscriber(&mut self, ws_id: String) {
        self.base.add_subscriber(ws_id)
    }

    fn remove_subscriber(&mut self, ws_id: String) {
        self.base.remove_subscriber(ws_id)
    }

    fn add_event_subscriber(&mut self, name: String, ws_id: String) {
        self.base.add_event_subscriber(name, ws_id)
    }

    fn remove_event_subscriber(&mut self, name: String, ws_id: String) {
        self.base.remove_event_subscriber(name, ws_id)
    }

    fn property_notify(&mut self, name: String, value: serde_json::Value) {
        self.base.property_notify(name, value)
    }

    fn action_notify(&mut self, action: serde_json::Map<String, serde_json::Value>) {
        self.base.action_notify(action)
    }

    fn event_notify(&mut self, name: String, event: serde_json::Map<String, serde_json::Value>) {
        self.base.event_notify(name, event)
    }

    fn start_action(&mut self, name: String, id: String) {
        self.base.start_action(name, id)
    }

    fn cancel_action(&mut self, name: String, id: String) {
        self.base.cancel_action(name, id)
    }

    fn finish_action(&mut self, name: String, id: String) {
        self.base.finish_action(name, id)
    }

    fn drain_queue(&mut self, ws_id: String) -> Vec<Drain<String>> {
        self.base.drain_queue(ws_id)
    }
}

struct Generator;

impl ActionGenerator for Generator {
    fn generate(
        &self,
        thing: Weak<RwLock<Box<Thing>>>,
        name: String,
        input: Option<&serde_json::Value>,
    ) -> Option<Box<Action>> {
        let input = match input {
            Some(v) => match v.as_object() {
                Some(o) => Some(o.clone()),
                None => None,
            },
            None => None,
        };

        let name: &str = &name;
        match name {
            "play" => Some(Box::new(PlayAction::new(input, thing))),
            "pause" => Some(Box::new(PauseAction::new(input, thing))),
            "stop" => Some(Box::new(StopAction::new(input, thing))),
            "next" => Some(Box::new(NextAction::new(input, thing))),
            "previous" => Some(Box::new(PreviousAction::new(input, thing))),
            "queueRandom" => Some(Box::new(QueueRandomAction::new(input, thing))),
            _ => None,
        }
    }
}

fn main() {
    let thing: Arc<RwLock<Box<Thing + 'static>>> = Arc::new(RwLock::new(Box::new(MPDThing::new())));
    let cloned = thing.clone();
    let mut last_playlist = "".to_owned();

    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_millis(1000));

        // Ideally, we should use `client.idle()`. However, there is no good way to _not_ block
        // while waiting for idle events. Instead, we'll just poll.

        let thing = cloned.clone();
        let mut thing = thing.write().unwrap();
        let thing = thing.as_mut_any().downcast_mut::<MPDThing>().unwrap();
        let client = thing.get_client();
        let mut client = client.write().unwrap();

        match MPDThing::get_playlist(&mut *client) {
            Ok(playlist) => {
                if playlist != last_playlist {
                    last_playlist = playlist.clone();
                    thing.add_event(Box::new(PlaylistUpdatedEvent::new(Some(json!(playlist)))));
                }
            }
            Err(_) => (),
        }

        match client.currentsong() {
            Ok(song) => {
                thing.update_artist(MPDThing::get_artist(&mut *client, &song));
                thing.update_album(MPDThing::get_album(&mut *client, &song));
                thing.update_title(MPDThing::get_title(&mut *client, &song));
            }
            Err(_) => (),
        }

        match client.status() {
            Ok(status) => {
                thing.update_volume(Some(MPDThing::get_volume(&mut *client, Some(&status))));
                thing.update_state(Some(MPDThing::get_state(&mut *client, Some(&status))));
            }
            Err(_) => (),
        }

        match client.status() {
            Ok(status) => {
                thing.update_repeat(Some(MPDThing::get_repeat(&mut *client, Some(&status))));
                thing.update_random(Some(MPDThing::get_random(&mut *client, Some(&status))));
            }
            Err(_) => (),
        }
    });

    let server = WebThingServer::new(
        ThingsType::Single(thing),
        Some(8888),
        None,
        None,
        Box::new(Generator),
    );
    server.start();
}
