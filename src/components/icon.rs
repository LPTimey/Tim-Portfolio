use crate::{Link, components::tooltip, link_logo};
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
    pub const fn img_link(self) -> Link {
        match self {
            Icon::AfterEffects => link_logo!("adobe-after-effects-svgrepo-com.svg"),
            Icon::Audacity => link_logo!("Audacity_Logo.svg"),
            Icon::Blender => link_logo!("blender-svgrepo-com.svg"),
            Icon::CreativeCloud => link_logo!("adobe-creative-cloud-svgrepo-com.svg"),
            Icon::Figma => link_logo!("figma-svgrepo-com.svg"),
            Icon::Illustrator => link_logo!("adobe-illustrator-svgrepo-com.svg"),
            Icon::Penpot => link_logo!("penpot-svgrepo-com.svg"),
            Icon::Photoshop => link_logo!("adobe-photoshop-svgrepo-com.svg"),
            Icon::Premiere => link_logo!("adobe-premiere-svgrepo-com.svg"),
            Icon::XD => link_logo!("adobe-xd-svgrepo-com.svg"),
            Icon::Houdini => link_logo!("houdini_logos/houdini_badge/houdini_badge_flat.svg"),
            Icon::Apple => link_logo!("apple-svgrepo-com.svg"),
            Icon::Excel => link_logo!("ms-excel-svgrepo-com.svg"),
            Icon::MacOS => link_logo!("macos-svgrepo-com.svg"),
            Icon::Microsoft => link_logo!("microsoft-svgrepo-com.svg"),
            Icon::Office => link_logo!("office-1-logo-svgrepo-com.svg"),
            Icon::PowerPoint => link_logo!("ms-powerpoint-svgrepo-com.svg"),
            Icon::Windows => link_logo!("microsoft-windows-svgrepo-com.svg"),
            Icon::Word => link_logo!("ms-word-svgrepo-com.svg"),
            Icon::Arduino => link_logo!("arduino-svgrepo-com.svg"),
            Icon::Bash => link_logo!("bash-icon-svgrepo-com.svg"),
            Icon::Bootstrap => link_logo!("bootstrap-svgrepo-com.svg"),
            Icon::C => link_logo!("c-svgrepo-com.svg"),
            Icon::Cpp => link_logo!("c-plusplus-svgrepo-com.svg"),
            Icon::CSharp => link_logo!("c-sharp-svgrepo-com.svg"),
            Icon::Css => link_logo!("CSS Logo.svg"),
            Icon::Docker => link_logo!("docker-svgrepo-com.svg"),
            Icon::Dribbble => link_logo!("dribbble-icon-svgrepo-com.svg"),
            Icon::Fedora => link_logo!("fedora-svgrepo-com.svg"),
            Icon::Fritzing => link_logo!("fritzing.svg"),
            Icon::Git => link_logo!("git-icon-svgrepo-com.svg"),
            Icon::GitHub => link_logo!("github-icon-svgrepo-com.svg"),
            Icon::Html => link_logo!("html-5-no-wordmark-svgrepo-com.svg"),
            Icon::Java => link_logo!("java-svgrepo-com.svg"),
            Icon::JavaScript => link_logo!("js-svgrepo-com.svg"),
            Icon::Jupyter => link_logo!("jupyter-svgrepo-com.svg"),
            Icon::Linux => link_logo!("linux-svgrepo-com.svg"),
            Icon::Lua => link_logo!("lua-svgrepo-com.svg"),
            Icon::NeoVim => link_logo!("neovim-mark@2x.svg"),
            Icon::Python => link_logo!("python-svgrepo-com.svg"),
            Icon::Rust => link_logo!("rust-svgrepo-com.svg"),
            Icon::TypeScript => link_logo!("typescript-icon-svgrepo-com.svg"),
            Icon::VSCode => link_logo!("vs-code-svgrepo-com.svg"),
            Icon::WebAssembly => link_logo!("webassembly-svgrepo-com.svg"),
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
                    img."no-border-r".(if self.invert_when_dark() {"invert-dark"} else {"test"})
                        width="40" height="40" src=(path_to_root.to_owned()+*img_link) alt="icon";
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
