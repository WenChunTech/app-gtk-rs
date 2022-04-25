use std::cell::Cell;
use std::rc::Rc;

use gtk::glib::clone;
// use gtk::glib::{clone, GString};
use gtk::glib;
use gtk::prelude::*;
use gtk::Button;
use gtk::{Application, ApplicationWindow};
fn main() {
    let app = Application::builder()
        .application_id("org.gtk.rs.demo")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let btn_increase = Button::builder()
        .label("Increase")
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .margin_top(12)
        .build();

    let btn_decrease = Button::builder()
        .label("Decrease")
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .margin_top(12)
        .build();

    let number = Rc::new(Cell::new(0));
    // let number_clone = number.clone();

    // btn_increase.connect_clicked(move |_| number.set(number.get() + 1));
    // btn_decrease.connect_clicked(move |_| number_clone.set(number_clone.get() - 1));
    btn_increase.connect_clicked(
        clone!(@weak number, @weak btn_decrease => move |_| {number.set(number.get() + 1);  btn_decrease.set_label(&number.get().to_string())}),
    );

    btn_decrease.connect_clicked(clone!(@weak btn_increase => move |_| number.set(number.get() - 1); btn_increase.set_label(&number.get().to_string())));
    // btn_decrease.connect_clicked(move |_| number_clone.set(number_clone.get() - 1));

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    gtk_box.append(&btn_increase);
    gtk_box.append(&btn_decrease);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK")
        .child(&gtk_box)
        .build();

    window.present();
}
