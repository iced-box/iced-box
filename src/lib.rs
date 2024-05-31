#[cfg(feature = "icon")]
mod icons;

#[cfg(feature = "toasts")]
mod toast;

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

#[cfg(feature = "toasts")]
pub mod toasts {
    //! Provides Toast alerts with title and body
    //! 
    //! ### Example of use
    //! 
    //! ```rust
    //! use icex_box::toasts::{danger, success};
    //! 
    //! self.toasts.push(danger("There was a problem"));
    //! 
    //! self.toasts.push(success("Good!").body("Success in taking the desired action"));
    //! 
    //! ```
    //! /// See an example of how to do this by [clicking here](https://github.com/iced-box/iced-box/tree/main/examples/toasts)

    pub use crate::toast::{Manager, Toast, helpers::*};
}
