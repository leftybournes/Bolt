/* window.rs
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

use adw::subclass::prelude::*;
use gtk::{
    gio::{self, ListStore},
    glib::{self, clone},
    prelude::*,
};

use crate::{
    data::{episode::object::EpisodeObject, show::object::ShowObject},
    discover::view::DiscoverView,
    empty::view::EmptyView,
    episodes::view::EpisodesView,
    podcasts,
    queue_view::QueueView,
    show_details::view::ShowDetails,
};

pub enum View {
    Discover,
    Empty,
    Loading,
    Podcasts,
    ShowDetails,
}

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/window.ui")]
    pub struct BoltWindow {
        #[template_child]
        pub main_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub btn_discover: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_refresh: TemplateChild<gtk::Button>,
        #[template_child]
        pub discover_view: TemplateChild<DiscoverView>,
        #[template_child]
        pub empty_view: TemplateChild<EmptyView>,
        #[template_child]
        pub queue_view: TemplateChild<QueueView>,
        #[template_child]
        pub episodes_view: TemplateChild<EpisodesView>,
        #[template_child]
        pub show_details_view: TemplateChild<ShowDetails>,
        #[template_child]
        pub podcasts_stack: TemplateChild<adw::ViewStack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BoltWindow {
        const NAME: &'static str = "BoltWindow";
        type Type = super::BoltWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BoltWindow {}
    impl WidgetImpl for BoltWindow {}
    impl WindowImpl for BoltWindow {}
    impl ApplicationWindowImpl for BoltWindow {}
    impl AdwApplicationWindowImpl for BoltWindow {}
}

glib::wrapper! {
    pub struct BoltWindow(ObjectSubclass<imp::BoltWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl BoltWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let window = glib::Object::builder::<BoltWindow>()
            .property("application", application)
            .build();

        window.connect_signals();
        window.load_shows();
        window.setup_discover();
        window.setup_episodes();

        window
    }

    pub fn show_view(&self, view: View) {
        let stack = self.imp().main_stack.get();

        match view {
            View::Empty => stack.set_visible_child_name("empty-view"),
            View::Loading => stack.set_visible_child_name("loading-view"),
            View::Podcasts => stack.set_visible_child_name("podcasts-view"),
            View::Discover => {
                stack.set_visible_child_name("discover-view");
            }
            View::ShowDetails => {
                stack.set_visible_child_name("show-details-view")
            }
        };
    }

    fn load_shows(&self) {
        self.show_view(View::Loading);

        glib::spawn_future_local(clone!(@weak self as window => async move {
            let shows = gio::spawn_blocking(move || podcasts::repository::load_show_count())
                .await
                .expect("Failed to load all shows");

            if shows > 0 {
                window.imp().episodes_view.get().load_episodes();
                window.show_view(View::Podcasts);
            } else {
                window.show_view(View::Empty);
            }
        }));
    }

    fn setup_discover(&self) {
        let model = ListStore::new::<ShowObject>();
        self.imp().discover_view.get().setup_model(&model);
    }

    fn setup_episodes(&self) {
        let model = ListStore::new::<EpisodeObject>();
        self.imp().episodes_view.get().setup_model(&model);
    }

    fn connect_signals(&self) {
        let imp = self.imp();

        imp.empty_view.btn_discover().connect_clicked(
            clone!(@weak self as window => move |_| {
                window.show_view(View::Discover);
            }),
        );

        imp.btn_discover.get().connect_clicked(
            clone!(@weak self as window => move |_| {
                window.show_view(View::Discover);
            }),
        );

        let discover_view = imp.discover_view.get();
        let discover_search_entry = discover_view.search_entry();

        discover_search_entry.connect_search_changed(
            move |entry: &gtk::SearchEntry| {
                if entry.text().len() > 3 {
                    discover_view.search_shows(&entry.text());
                }
            },
        );

        imp.podcasts_stack.get().connect_visible_child_notify(
            clone!(@weak imp => move |stack| {
                if let Some(name) = stack.visible_child_name() {
                    let name = name.as_str();

                    if name == "episodes" {
                        imp.btn_refresh.get().set_visible(true);
                    } else {
                        imp.btn_refresh.get().set_visible(false);
                    }
                }
            }),
        );

        imp.btn_refresh
            .get()
            .connect_clicked(clone!(@weak imp => move |_| {
                imp.episodes_view.get().load_episodes();
            }));

        let discover_view = imp.discover_view.get();

        discover_view.search_results().connect_child_activated(
            clone!(@weak self as window, @weak discover_view => move |_container, child| {
                if let Some (ref model) = *discover_view.imp().model.borrow() {
                    let index: u32 = child.index().try_into().expect("Index cannot be out of range");
                    let show = model.item(index).and_downcast::<ShowObject>();

                    if let Some(show) = show {
                        window.imp().show_details_view.get().load_details(&show);
                        window.show_view(View::ShowDetails);
                    }
                };
            })
        );

        discover_view.back_button().connect_clicked(
            clone!(@weak self as window => move |_| {
                window.show_view(View::Podcasts);
            }),
        );

        self.imp()
            .show_details_view
            .get()
            .back_button()
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.show_view(View::Discover);
            }));
    }
}
