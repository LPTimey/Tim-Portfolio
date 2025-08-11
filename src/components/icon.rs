use crate::{Link, link_logo};
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
    CSS,
    Docker,
    Dribbble,
    HTML,
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
    pub const fn data(self) -> (&'static str, Link) {
        match self {
            Icon::AfterEffects => (
                "Adobe After Effects",
                link_logo!("adobe-after-effects-svgrepo-com.svg"),
            ),
            Icon::Audacity => ("Audacity", link_logo!("Audacity_Logo.svg")),
            Icon::Blender => ("Blender", link_logo!("blender-svgrepo-com.svg")),
            Icon::CreativeCloud => (
                "Adobe Creative Cloud",
                link_logo!("adobe-creative-cloud-svgrepo-com.svg"),
            ),
            Icon::Figma => ("Figma", link_logo!("figma-svgrepo-com.svg")),
            Icon::Illustrator => (
                "Adobe Illustrator",
                link_logo!("adobe-illustrator-svgrepo-com.svg"),
            ),
            Icon::Penpot => ("Penpot", link_logo!("penpot-svgrepo-com.svg")),
            Icon::Photoshop => (
                "Adobe Photoshop",
                link_logo!("adobe-photoshop-svgrepo-com.svg"),
            ),
            Icon::Premiere => (
                "Adobe Premiere Pro",
                link_logo!("adobe-premiere-svgrepo-com.svg"),
            ),
            Icon::XD => ("Adobe XD", link_logo!("adobe-xd-svgrepo-com.svg")),
            Icon::Houdini => ("Houdini", link_logo!("houdini_logos/houdini_badge/houdini_badge_flat.svg")),
            Icon::Apple => ("Apple", link_logo!("apple-svgrepo-com.svg")),
            Icon::Excel => ("Microsoft Excel", link_logo!("ms-excel-svgrepo-com.svg")),
            Icon::MacOS => ("macOS", link_logo!("macos-svgrepo-com.svg")),
            Icon::Microsoft => ("Microsoft", link_logo!("microsoft-svgrepo-com.svg")),
            Icon::Office => (
                "Microsoft Office",
                link_logo!("office-1-logo-svgrepo-com.svg"),
            ),
            Icon::PowerPoint => (
                "Microsoft PowerPoint",
                link_logo!("ms-powerpoint-svgrepo-com.svg"),
            ),
            Icon::Windows => ("Windows", link_logo!("microsoft-windows-svgrepo-com.svg")),
            Icon::Word => ("Microsoft Word", link_logo!("ms-word-svgrepo-com.svg")),
            Icon::Arduino => ("Arduino", link_logo!("arduino-svgrepo-com.svg")),
            Icon::Bash => ("Bash", link_logo!("bash-icon-svgrepo-com.svg")),
            Icon::Bootstrap => ("Bootstrap", link_logo!("bootstrap-svgrepo-com.svg")),
            Icon::C => ("C", link_logo!("c-svgrepo-com.svg")),
            Icon::Cpp => ("C++", link_logo!("c-plusplus-svgrepo-com.svg")),
            Icon::CSharp => ("C#", link_logo!("c-sharp-svgrepo-com.svg")),
            Icon::CSS => ("CSS", link_logo!("css-3-svgrepo-com.svg")),
            Icon::Docker => ("Docker", link_logo!("docker-svgrepo-com.svg")),
            Icon::Dribbble => ("Dribbble", link_logo!("dribbble-icon-svgrepo-com.svg")),
            Icon::Fedora => ("Fedora", link_logo!("fedora-svgrepo-com.svg")),
            Icon::Fritzing => ("Fritzing", link_logo!("fritzing.svg")),
            Icon::Git => ("Git", link_logo!("git-icon-svgrepo-com.svg")),
            Icon::GitHub => ("GitHub", link_logo!("github-icon-svgrepo-com.svg")),
            Icon::HTML => ("HTML", link_logo!("html-5-svgrepo-com.svg")),
            Icon::Java => ("Java", link_logo!("java-svgrepo-com.svg")),
            Icon::JavaScript => ("JavaScript", link_logo!("js-svgrepo-com.svg")),
            Icon::Jupyter => ("Jupyter", link_logo!("jupyter-svgrepo-com.svg")),
            Icon::Linux => ("Linux", link_logo!("linux-svgrepo-com.svg")),
            Icon::Lua => ("Lua", link_logo!("lua-svgrepo-com.svg")),
            Icon::NeoVim => ("NeoVim", link_logo!("neovim-svgrepo-com.svg")),
            Icon::Python => ("Python", link_logo!("python-svgrepo-com.svg")),
            Icon::Rust => ("Rust", link_logo!("rust-svgrepo-com.svg")),
            Icon::TypeScript => ("TypeScript", link_logo!("typescript-icon-svgrepo-com.svg")),
            Icon::VSCode => ("VS Code", link_logo!("vs-code-svgrepo-com.svg")),
            Icon::WebAssembly => ("WebAssembly", link_logo!("webassembly-svgrepo-com.svg")),
        }
    }
}
