/* object.rs
 *
 * Copyright 2024 Kent Delante
 *
 * This file is part of Bolt.
 *
 * Bolt is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Bolt is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *https://api.podcastindex.org/api/1.0/recent/feeds?pretty
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use std::cell::RefCell;

use adw::prelude::*;
use gtk::glib::{self, subclass::prelude::*, Properties};

use crate::{api::episode::Episode as ApiEpisode, data::episode::Episode};

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::EpisodeObject)]
    pub struct EpisodeObject {
        #[property(name = "id", get, construct_only, type = i64, member = id)]
        #[property(name = "title", get, construct_only, type = Option<String>, member = title)]
        #[property(name = "description", get, construct_only, type = Option<String>, member = description)]
        #[property(name = "url", get, construct_only, type = Option<String>, member = url)]
        #[property(name = "image-url", get, construct_only, type = Option<String>, member = image_url)]
        #[property(name = "media-url", get, construct_only, type = String, member = media_url)]
        #[property(name = "queued", get, construct_only, type = i64, member = queued)]
        #[property(name = "date-published", get, construct_only, type = i64, member = date_published)]
        #[property(name = "show-id", get, construct_only, type = i64, member = show_id)]
        data: RefCell<Episode>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EpisodeObject {
        const NAME: &'static str = "EpisodeObject";
        type Type = super::EpisodeObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for EpisodeObject {}
}

glib::wrapper! {
    pub struct EpisodeObject(ObjectSubclass<imp::EpisodeObject>);
}

impl Default for EpisodeObject {
    fn default() -> Self {
        glib::Object::builder::<Self>().build()
    }
}

impl From<ApiEpisode> for EpisodeObject {
    fn from(episode: ApiEpisode) -> Self {
        glib::Object::builder::<Self>()
            .property("id", episode.id)
            .property("title", Some(episode.title))
            .property(
                "description",
                Some(episode.description).filter(|text| !text.is_empty()),
            )
            .property("url", Some(episode.link).filter(|url| !url.is_empty()))
            .property(
                "image-url",
                Some(episode.image).filter(|image| !image.is_empty()),
            )
            .property("media-url", episode.enclosure_url)
            .property("queued", 0)
            .property("date-published", episode.date_published)
            .property("show-id", episode.feed_id)
            .build()
    }
}

impl From<Episode> for EpisodeObject {
    fn from(episode: Episode) -> Self {
        glib::Object::builder::<Self>()
            .property("id", episode.id)
            .property("title", episode.title)
            .property("description", episode.description)
            .property("url", episode.url)
            .property("image-url", episode.image_url)
            .property("media-url", episode.media_url)
            .property("queued", episode.queued)
            .property("date-published", episode.date_published)
            .property("show-id", episode.show_id)
            .build()
    }
}