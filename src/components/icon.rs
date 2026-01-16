use std::sync::Arc;

use crate::{
    assets::{
        svg::{Svg, SvgProps},
    },
    components::tooltip,
};
use maud::{Markup, html};
use strum::Display;

#[allow(unused)]
#[derive(Debug, Display, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Icon {
    // 🎨 Design & KreativSoftware
    AfterEffects,
    Audacity,
    Blender,
    CreativeCloud,
    Figma,
    Houdini,
    Illustrator,
    Penpot,
    Photoshop,
    Premiere,
    XD,

    // 💻 Allgemeine EDV & Tools
    Apple,
    Excel,
    MacOS,
    Fedora,
    Fritzing,
    Git,
    GitHub,
    Linux,
    Microsoft,
    NeoVim,
    Office,
    PowerPoint,
    VSCode,
    Windows,
    Word,

    // 🔧 Programmiersprachen
    Arduino,
    Bash,
    Bootstrap,
    C,
    Cpp,
    CSharp,
    Css,
    Docker,
    Dribbble,
    Html,
    Java,
    JavaScript,
    Jupyter,
    Lua,
    Python,
    Rust,
    TypeScript,
    WebAssembly,
}

impl Icon {
    pub const fn name(self) -> &'static str {
        match self {
            Icon::AfterEffects => "Adobe After Effects",
            Icon::Audacity => "Audacity",
            Icon::Blender => "Blender",
            Icon::CreativeCloud => "Adobe Creative Cloud",
            Icon::Figma => "Figma",
            Icon::Illustrator => "Adobe Illustrator",
            Icon::Penpot => "Penpot",
            Icon::Photoshop => "Adobe Photoshop",
            Icon::Premiere => "Adobe Premiere Pro",
            Icon::XD => "Adobe XD",
            Icon::Houdini => "Houdini",
            Icon::Apple => "Apple",
            Icon::Excel => "Microsoft Excel",
            Icon::MacOS => "macOS",
            Icon::Microsoft => "Microsoft",
            Icon::Office => "Microsoft Office",
            Icon::PowerPoint => "Microsoft PowerPoint",
            Icon::Windows => "Windows",
            Icon::Word => "Microsoft Word",
            Icon::Arduino => "Arduino",
            Icon::Bash => "Bash",
            Icon::Bootstrap => "Bootstrap",
            Icon::C => "C",
            Icon::Cpp => "C++",
            Icon::CSharp => "C#",
            Icon::Css => "CSS",
            Icon::Docker => "Docker",
            Icon::Dribbble => "Dribbble",
            Icon::Fedora => "Fedora",
            Icon::Fritzing => "Fritzing",
            Icon::Git => "Git",
            Icon::GitHub => "GitHub",
            Icon::Html => "HTML",
            Icon::Java => "Java",
            Icon::JavaScript => "JavaScript",
            Icon::Jupyter => "Jupyter",
            Icon::Linux => "Linux",
            Icon::Lua => "Lua",
            Icon::NeoVim => "NeoVim",
            Icon::Python => "Python",
            Icon::Rust => "Rust",
            Icon::TypeScript => "TypeScript",
            Icon::VSCode => "VS Code",
            Icon::WebAssembly => "WebAssembly",
        }
    }
    pub fn img_link(self) -> Arc<Svg> {
        match self {
            Icon::AfterEffects => Svg::new(
                "public",
                "assets/logos/adobe-after-effects-svgrepo-com.svg",
                "",
            )
            .expect("Expected Valid Link To File"),
            Icon::Audacity => Svg::new("public", "assets/logos/Audacity_Logo.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Blender => Svg::new("public", "assets/logos/blender-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::CreativeCloud => Svg::new(
                "public",
                "assets/logos/adobe-creative-cloud-svgrepo-com.svg",
                "",
            )
            .expect("Expected Valid Link To File"),
            Icon::Figma => Svg::new("public", "assets/logos/figma-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Illustrator => Svg::new(
                "public",
                "assets/logos/adobe-illustrator-svgrepo-com.svg",
                "",
            )
            .expect("Expected Valid Link To File"),
            Icon::Penpot => Svg::new("public", "assets/logos/penpot-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Photoshop => {
                Svg::new("public", "assets/logos/adobe-photoshop-svgrepo-com.svg", "")
                    .expect("Expected Valid Link To File")
            }
            Icon::Premiere => Svg::new("public", "assets/logos/adobe-premiere-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::XD => Svg::new("public", "assets/logos/adobe-xd-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Houdini => Svg::new(
                "public",
                "assets/logos/houdini_logos/houdini_badge/houdini_badge_flat.svg",
                "",
            )
            .expect("Expected Valid Link To File"),
            Icon::Apple => Svg::new("public", "assets/logos/apple-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Excel => Svg::new("public", "assets/logos/ms-excel-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::MacOS => Svg::new("public", "assets/logos/macos-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Microsoft => Svg::new("public", "assets/logos/microsoft-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Office => Svg::new("public", "assets/logos/office-1-logo-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::PowerPoint => {
                Svg::new("public", "assets/logos/ms-powerpoint-svgrepo-com.svg", "")
                    .expect("Expected Valid Link To File")
            }
            Icon::Windows => Svg::new(
                "public",
                "assets/logos/microsoft-windows-svgrepo-com.svg",
                "",
            )
            .expect("Expected Valid Link To File"),
            Icon::Word => Svg::new("public", "assets/logos/ms-word-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Arduino => Svg::new("public", "assets/logos/arduino-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Bash => Svg::new("public", "assets/logos/bash-icon-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Bootstrap => Svg::new("public", "assets/logos/bootstrap-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::C => Svg::new("public", "assets/logos/c-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Cpp => Svg::new("public", "assets/logos/c-plusplus-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::CSharp => Svg::new("public", "assets/logos/c-sharp-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Css => Svg::new("public", "assets/logos/CSS Logo.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Docker => Svg::new("public", "assets/logos/docker-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Dribbble => Svg::new("public", "assets/logos/dribbble-icon-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Fedora => Svg::new("public", "assets/logos/fedora-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Fritzing => Svg::new("public", "assets/logos/fritzing.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Git => Svg::new("public", "assets/logos/git-icon-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::GitHub => Svg::new("public", "assets/logos/github-icon-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Html => Svg::new(
                "public",
                "assets/logos/html-5-no-wordmark-svgrepo-com.svg",
                "",
            )
            .expect("Expected Valid Link To File"),
            Icon::Java => Svg::new("public", "assets/logos/java-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::JavaScript => Svg::new("public", "assets/logos/js-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Jupyter => Svg::new("public", "assets/logos/jupyter-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Linux => Svg::new("public", "assets/logos/linux-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Lua => Svg::new("public", "assets/logos/lua-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::NeoVim => Svg::new("public", "assets/logos/neovim-mark@2x.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Python => Svg::new("public", "assets/logos/python-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::Rust => Svg::new("public", "assets/logos/rust-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::TypeScript => {
                Svg::new("public", "assets/logos/typescript-icon-svgrepo-com.svg", "")
                    .expect("Expected Valid Link To File")
            }
            Icon::VSCode => Svg::new("public", "assets/logos/vs-code-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
            Icon::WebAssembly => Svg::new("public", "assets/logos/webassembly-svgrepo-com.svg", "")
                .expect("Expected Valid Link To File"),
        }
    }
    pub const fn site_link(self) -> &'static str {
        match self {
            Icon::AfterEffects => "https://www.adobe.com/products/aftereffects.html",
            Icon::Audacity => "https://www.audacityteam.org/",
            Icon::Blender => "https://www.blender.org/",
            Icon::CreativeCloud => "https://www.adobe.com/creativecloud.html",
            Icon::Figma => "https://www.figma.com/",
            Icon::Houdini => "https://www.sidefx.com/",
            Icon::Illustrator => "https://www.adobe.com/products/illustrator.html",
            Icon::Penpot => "https://penpot.app/",
            Icon::Photoshop => "https://www.adobe.com/products/photoshop.html",
            Icon::Premiere => "https://www.adobe.com/products/premiere.html",
            Icon::XD => "https://www.adobe.com/products/xd.html",
            Icon::Apple => "https://www.apple.com/",
            Icon::Excel => "https://www.microsoft.com/en-us/microsoft-365/excel",
            Icon::MacOS => "https://www.apple.com/macos/",
            Icon::Fedora => "https://getfedora.org/",
            Icon::Fritzing => "https://fritzing.org/",
            Icon::Git => "https://git-scm.com/",
            Icon::GitHub => "https://github.com/",
            Icon::Linux => "https://www.kernel.org/",
            Icon::Microsoft => "https://www.microsoft.com/",
            Icon::NeoVim => "https://neovim.io/",
            Icon::Office => "https://www.microsoft.com/en-us/microsoft-365",
            Icon::PowerPoint => "https://www.microsoft.com/en-us/microsoft-365/powerpoint",
            Icon::VSCode => "https://code.visualstudio.com/",
            Icon::Windows => "https://www.microsoft.com/windows/",
            Icon::Word => "https://www.microsoft.com/en-us/microsoft-365/word",
            Icon::Arduino => "https://www.arduino.cc/",
            Icon::Bash => "https://www.gnu.org/software/bash/",
            Icon::Bootstrap => "https://getbootstrap.com/",
            Icon::C => "https://www.iso.org/standard/82075.html",
            // Icon::C => "https://en.wikipedia.org/wiki/C_(programming_language)",
            Icon::Cpp => "https://isocpp.org/",
            Icon::CSharp => "https://learn.microsoft.com/en-us/dotnet/csharp/",
            Icon::Css => "https://developer.mozilla.org/en-US/docs/Web/CSS",
            Icon::Docker => "https://www.docker.com/",
            Icon::Dribbble => "https://dribbble.com/",
            Icon::Html => "https://developer.mozilla.org/en-US/docs/Web/HTML",
            Icon::Java => "https://www.java.com/de/",
            Icon::JavaScript => "https://developer.mozilla.org/en-US/docs/Web/JavaScript",
            Icon::Jupyter => "https://jupyter.org/",
            Icon::Lua => "https://www.lua.org/",
            Icon::Python => "https://www.python.org/",
            Icon::Rust => "https://www.rust-lang.org/",
            Icon::TypeScript => "https://www.typescriptlang.org/",
            Icon::WebAssembly => "https://webassembly.org/",
        }
    }
    pub const fn invert_when_dark(&self) -> bool {
        match self {
            Icon::AfterEffects => false,
            Icon::Audacity => false,
            Icon::Blender => false,
            Icon::CreativeCloud => false,
            Icon::Figma => false,
            Icon::Houdini => false,
            Icon::Illustrator => false,
            Icon::Penpot => true,
            Icon::Photoshop => false,
            Icon::Premiere => false,
            Icon::XD => false,
            Icon::Apple => false,
            Icon::Excel => false,
            Icon::MacOS => false,
            Icon::Fedora => false,
            Icon::Fritzing => false,
            Icon::Git => false,
            Icon::GitHub => true,
            Icon::Linux => false,
            Icon::Microsoft => false,
            Icon::NeoVim => false,
            Icon::Office => false,
            Icon::PowerPoint => false,
            Icon::VSCode => false,
            Icon::Windows => false,
            Icon::Word => false,
            Icon::Arduino => false,
            Icon::Bash => false,
            Icon::Bootstrap => false,
            Icon::C => false,
            Icon::Cpp => false,
            Icon::CSharp => false,
            Icon::Css => false,
            Icon::Docker => false,
            Icon::Dribbble => false,
            Icon::Html => false,
            Icon::Java => false,
            Icon::JavaScript => false,
            Icon::Jupyter => false,
            Icon::Lua => false,
            Icon::Python => false,
            Icon::Rust => true,
            Icon::TypeScript => false,
            Icon::WebAssembly => false,
        }
    }
}
pub trait IconToMarkup {
    fn to_markup(&self, path_to_root: &str) -> Markup;
}
impl IconToMarkup for [Icon] {
    fn to_markup(&self, path_to_root: &str) -> Markup {
        html! {
            @for icon in self {
                li ."skill-icon"{
                    (icon.to_markup(path_to_root))
                }
            }
        }
    }
}
impl IconToMarkup for Icon {
    fn to_markup(&self, path_to_root: &str) -> Markup {
        let (img_link, name, site_link) = (self.img_link(), self.name(), self.site_link());
        html!(
            (tooltip::markup(tooltip::MarkupProps {
                children: html! {
                    // img."no-border-r".(if self.invert_when_dark() {"invert-dark"} else {"test"})
                    //     width="40" height="40" src=(path_to_root.to_owned()+*img_link) alt="icon";
                    (img_link.render(SvgProps{path_to_root:path_to_root,class:&["no-border-r",(if self.invert_when_dark() {"invert-dark"} else {""})],..Default::default()}))
                },
                content: html!(a.link.underline target="_blank" href=(site_link) {(name)}),
                popup_align: tooltip::Align::Center,
                popup_justify: tooltip::Align::End,
                popup_begin_justify: tooltip::Align::Center,
                popup_begin_align: tooltip::Align::Center
            }))
        )
    }
}
