use gtk::glib::{self, object_subclass, wrapper, Properties};
use gtk::{prelude::*, subclass::prelude::*};
use std::cell::RefCell;

wrapper! {
    pub struct Window(ObjectSubclass<WindowPriv>)
    @extends gtk::Window, gtk::Bin, gtk::Container, gtk::Widget, @implements gtk::Buildable;
}

#[derive(Properties)]
#[properties(wrapper_type = Window)]
pub struct WindowPriv {
    #[property(get, name = "x", nick = "X", blurb = "Global x coordinate", default = 0)]
    x: RefCell<i32>,

    #[property(get, name = "y", nick = "Y", blurb = "Global y coordinate", default = 0)]
    y: RefCell<i32>,

    // Only set after the first drag. This is never persisted across window lifetimes.
    drag_position: RefCell<Option<(i32, i32)>>,
}

// This should match the default values from the ParamSpecs
impl Default for WindowPriv {
    fn default() -> Self {
        WindowPriv { x: RefCell::new(0), y: RefCell::new(0), drag_position: RefCell::new(None) }
    }
}

#[object_subclass]
impl ObjectSubclass for WindowPriv {
    type ParentType = gtk::Window;
    type Type = Window;

    const NAME: &'static str = "WindowEww";
}

impl Default for Window {
    fn default() -> Self {
        glib::Object::new::<Self>()
    }
}

impl Window {
    pub fn new(type_: gtk::WindowType, x_: i32, y_: i32) -> Self {
        let w: Self = glib::Object::builder().property("type", type_).build();
        let priv_ = w.imp();
        priv_.x.replace(x_);
        priv_.y.replace(y_);
        w
    }

    /// Initial top-left monitor-relative coordinates, as computed from the Yuck geometry.
    pub fn initial_drag_position(&self) -> (i32, i32) {
        let private = self.imp();
        (*private.x.borrow(), *private.y.borrow())
    }

    /// None until the user begins dragging this particular window instance.
    pub fn drag_position(&self) -> Option<(i32, i32)> {
        *self.imp().drag_position.borrow()
    }

    pub fn set_drag_position(&self, x: i32, y: i32) {
        self.imp().drag_position.replace(Some((x, y)));
    }
}

impl ObjectImpl for WindowPriv {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        self.derived_property(id, pspec)
    }
}
impl WindowImpl for WindowPriv {}
impl BinImpl for WindowPriv {}
impl ContainerImpl for WindowPriv {}
impl WidgetImpl for WindowPriv {}
