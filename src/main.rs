use gtk::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Test GtkSearchEntry bug");
    window.set_default_size(300, 300);

    let button = Button::new_with_label("Click me!");
    window.add(&button);

    const SRC: &str = include_str!("ui/find_replace.glade");

    let builder = Builder::new_from_string(SRC);
    let search_bar: SearchBar = builder.get_object("search_bar").unwrap();
    let popover: Popover = builder.get_object("search_popover").unwrap();

    popover.set_position(PositionType::Top);
    popover.set_relative_to(&button);

    window.show_all();

    button.connect_clicked(move |_| {
        search_bar.set_search_mode(true);
        popover.show();
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
