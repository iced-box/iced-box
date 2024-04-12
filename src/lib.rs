#[cfg(feature = "lucide_icons")]
extern crate icons_lucide;

#[cfg(feature = "material_icons")]
extern crate icons_material;

//#[cfg(feature = "icon")]
pub mod icon {
    //!
    //! Provides Lucide and Google Material style icons 
    //! 
    //! ![](https://raw.githubusercontent.com/iced-box/iced-box/main/examples/icons/image.png)
    /// 
    ///
    #[cfg(feature = "lucide_icons")]
    pub mod lucide {
        //! Provides Lucide style icons 
        pub use icons_lucide::*;
    }

    #[cfg(feature = "material_icons")]
    pub mod material {
        //! Provides Google Material style icons 
        pub use icons_material::*;
    }

    
    /// Alias for the result of the font load command
    #[cfg(feature = "icon")]
    pub type LoadingResult = Result<(), iced::font::Error>;
}
