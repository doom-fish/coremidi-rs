use coremidi::prelude::*;

fn main() {
    if let Some(endpoint) = sources().next().or_else(|| destinations().next()) {
        println!(
            "endpoint unique_id={:?} display_name={:?}",
            endpoint.unique_id().ok(),
            endpoint.string_property(MidiProperty::display_name()).ok(),
        );
    } else {
        println!("property constant name={:?}", MidiProperty::name());
    }
}
