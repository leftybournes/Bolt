/* card.rs
 *
 * Copyright 2023 Kent Delante
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
 *
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 */

use gtk::{
    gio,
    glib,
    subclass::prelude::*,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/explore/card.ui")]
    pub struct ExploreCard {
        #[template_child]
        pub picture_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub image_missing_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub name: TemplateChild<gtk::Label>,
        #[template_child]
        pub description: TemplateChild<gtk::Label>,
        #[template_child]
        pub unsubscribe_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub subscribe_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExploreCard {
        const NAME: &'static str = "ExploreCard";
        type Type = super::ExploreCard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExploreCard {}
    impl WidgetImpl for ExploreCard {}
    impl BoxImpl for ExploreCard {}
}

glib::wrapper! {
    pub struct ExploreCard(ObjectSubclass<imp::ExploreCard>)
        @extends gtk::Widget, gtk::Box,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for ExploreCard {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ExploreCard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe_button(&self) -> gtk::Button {
        self.imp().subscribe_button.get()
    }

    pub fn unsubscribe_button(&self) -> gtk::Button {
        self.imp().unsubscribe_button.get()
    }

    pub fn set_name(&self, name: &str) {
        self.imp().name.set_text(name);
    }

    pub fn set_description(&self, content: &str) {
        self.imp().description.set_text(content);
    }

    pub fn load_image(&self, id: &i64, image_url: &str) {}
    pub fn subscribe(&self, id: &i64) {}
    pub fn unsubscribe(&self, id: &i64) {}
    fn show_image_missing_icon(&self) {}
    fn show_cover(&self) {}
}
