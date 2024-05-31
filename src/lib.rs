#[cfg(feature = "icon")]
mod icons;

#[cfg(feature = "icon")]
pub mod icon {
    //!
    //! Provides Lucide and Google Material style icons 
    //! 
    //! ![](https://raw.githubusercontent.com/iced-box/iced-box/main/examples/icons/image.png)
    /// 
    ///
    ///  Provides Lucide style icons 
    #[cfg(feature = "lucide_icons")]
    pub use crate::icons::lucide;
        
    /// Provides Google Material style icons 
    #[cfg(feature = "material_icons")]
    pub use crate::icons::material;
    
    /// Alias for the result of the font load command
    #[cfg(feature = "icon")]
    pub type LoadingResult = Result<(), iced::font::Error>;
}

mod toasts;
