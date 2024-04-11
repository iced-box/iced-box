//! Provides Lucide and Google Material style icons 
//! 
//! ![](https://github.com/iced-box/iced-box/blob/main/examples/icons/image.png)
/// 
///
mod lucide;
mod material;


/// A module that allows the use of [Lucide](https://lucide.dev/icons) icons in a simplified way, see all the icons that can be used on the official page <https://lucide.dev/icons>
pub use lucide::{Lucide, load_lucide_font, lucide_font};

/// A module that allows the use of [Material](https://fonts.google.com/icons) icons in a simplified way, see all the icons that can be used on the official page <https://fonts.google.com/icons>
pub use material::{Material, load_material_font, material_font};
