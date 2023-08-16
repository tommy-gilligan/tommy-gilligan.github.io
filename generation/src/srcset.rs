use url::Url;

const DENSITIES: [u16; 3] = [1, 2, 3];

pub fn srcset<F>(image_url_for: F, base_size: u16) -> String
where
    F: Fn(u16) -> Url,
{
    DENSITIES
        .into_iter()
        .map(|density| format!("{} {}x", image_url_for(density * base_size), density))
        .reduce(|memo, obj| format!("{}, {}", memo, obj))
        .unwrap()
}
