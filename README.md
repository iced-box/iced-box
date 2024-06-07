<img src="https://raw.githubusercontent.com/iced-rs/iced/master/docs/logo.svg" width="140px" />

# iced-box
A utility box for iced.rs that provides toasts notifications, material icons, lucide and more.

### Useful links
iced-box documentation [here](https://docs.rs/iced-box/latest/iced_box/)

### Installation
To install a single feature, use ```cargo add iced-box --features material_icons```\
To install several features, use ```cargo add iced-box --features material_icons,lucide_icons``` 

### Compatibility with iced versions
| **iced** version | **iced-box** version |
| --- |----------------------|
| 0.13.0-dev | 0.6                  | 
| 0.12 | 0.5, 0.3, 0.2        |
| 0.10 | 0.4                  |

### Attention
To use version 0.6, you must use the following version of iced in your Cargo.toml dependencies:
```
iced = {  git = "https://github.com/iced-rs/iced.git", rev = "06ff17fcf87495663a295d1548df1c2ac03dafbd" }
```
*At the moment I have no plans to add backwards compatibility to iced for new features,
Feel free to send a PR for this purpose, it would be greatly appreciated!*

### Features
| Name | Description |
| --- | --- |
| [material_icons](https://docs.rs/iced-box/latest/iced_box/icon/material) | Provides Google Material style icons  |
| [lucide_icons](https://docs.rs/iced-box/latest/iced_box/icon/lucide) | Provides Lucide style icons |
| [toasts](https://docs.rs/iced-box/latest/iced_box/toasts) | Provides alerts and toast messages |

### Description
This box was created with the aim of offering new features to [iced](https://iced.rs)
Some code examples are expected here too
Today, iced has a complicated learning curve, one of the objectives of this project is to collaborate with more beginner users in this iced world


### For the future
| Functionality | Description |
| --- | --- |
| table | something similar to [this](https://getbootstrap.com/docs/4.0/content/tables/) |


### Contributing
Are you interested in contributing to the project? [know more](CONTRIBUTING.md) 

### Donation
| Crypto | Address                             |
| --- |-------------------------------------|
| Dogecoin | DD3aGVrzfNDMkeE2Gc7rHLjbmdwMV4DBqG  |
