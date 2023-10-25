use figlet_rs::FIGfont;
use mendeleev::{Element, Isotope, Percent};

fn main() {
    if let Ok(standard_font) = FIGfont::standard() {
        if let Some(figure) = standard_font.convert("Rusty world!") {
            println!("{}", figure)
        }
    }

    println!("Isotopes of Hydrogen:");
    let hydrogen = Element::H;
    let isotopes = Isotope::list()
        .iter()
        .filter_map(|iso| {
            if iso.element() == hydrogen {
                Some(iso)
            } else {
                None
            }
        })
        .collect::<Vec<&Isotope>>();

    let mut other_isotope: String;
    for isotope in isotopes {
        let name = match isotope {
            Isotope::H1 => "Protium",
            Isotope::H2 => "Deuterium",
            Isotope::H3 => "Tritium",
            _ => {
                other_isotope = isotope.display_with_name();
                &other_isotope
            }
        };

        println!(
            "  ⦿ {} ({})",
            name,
            isotope.natural_abundance().unwrap_or(Percent(0.0)),
        );
    }
}