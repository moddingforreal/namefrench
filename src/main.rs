use std::env;

fn main() {
    let mut store_str = String::from(" ");
    let frenchname: String =
        env::args().skip(1).map(|s| s.chars().map(
            |c| match c.to_lowercase().next()
                .unwrap_or_else(|| c) {
                    'a' => "ée",
                    'b' => "bou",
                    'c' => "cie",
                    'd' => "Deau",
                    'e' => "Euaoix",
                    'f' => "feu",
                    'g' => "Garçon",
                    'h' => "VLE CON D'TA GRAND-MERE",
                    'i' => "Iàèìòù",
                    'j' => "j",
                    'k' => "Kours",
                    'l' => "🇱",
                    'm' => "Meu (comme une vache xptdrrr)",
                    'n' => "💛𝕤𝕠𝕝𝕖𝕚𝕝💛",
                    'o' => "974",
                    'p' => "pet",
                    'q' => "𝒷𝑜𝓊𝓁𝑒 𝒹' 𝑒𝓃𝑒𝓇𝑔𝒾𝑒",
                    'r' => "r",
                    's' => "Jean-pierre",
                    't' => "Ouireuxleaussielledeuroyeauou",
                    'u' => "Ue",
                    'v' => "Vursilitée",
                    'w' => "2€",
                    'x' => "emmanuel macron",
                    'y' => "i grec",
                    'z' => "pédale",
                    _ => {
                        store_str = c.to_string();
                        &store_str[..]
                    },
                }.to_string()).collect::<Vec<_>>().concat())
            .collect::<Vec<_>>().concat();
    println!("{}", frenchname);
}
