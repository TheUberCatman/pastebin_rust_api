pub enum Format {
    CS4,
    ACMECrossAssembler6502,
    KickAssembler6502,
    TASM64TASS6502 ,
    ABAP,
    ActionScript,
    ActionScript3,
    Ada,
    AIMMS,
    ALGOL68,
    ApacheLog,
    AppleScript,
    APTSources,
    ARM,
    ASMNASM,
    ASP,
    Asymptote,
    Autoconf,
    Autohotkey,
    AutoIt,
    Avisynth,
    Awk,
    BASCOMAVR,
    Bash,
    Basic4GL,
    Batch,
    BibTeX,
    BlitzBasic,
    Blitz3D,
    BlitzMax,
    BNF,
    BOO,
    BrainFuck,
    C,
    CWinAPI,
    CMacs,
    CIntermediateLanguage,
    CSharp,
    CPlusPlus,
    CPlusPlusWinAPI,
    CPlusPlusQt,
    CLoadrunner,
    CADDCL,
    CADLisp,
    Ceylon,
    CFDG,
    ChaiScript,
    Chapel,
    Clojure,
    CloneC,
    CloneCPlusPlus,
    CMake,
    COBOL,
    CoffeeScript,
    ColdFusion,
    CSS,
    Cuesheet,
    D,
    Dart,
    DCL,
    DCPU16,
    DCS,
    Delphi,
    DelphiPrismOxygene,
    Diff,
    DIV,
    DOT,
    E,
    Easytrieve,
    ECMAScript,
    Eiffel,
    Email,
    EPC,
    Erlang,
    Euphoria,
    FSharp,
    Falcon,
    Filemaker,
    FOLanguage,
    FormulaOne,
    Fortran,
    FreeBasic,
    FreeSWITCH,
    GAMBAS,
    GameMaker,
    GDB,
    Genero,
    Genie,
    GetText,
    Go,
    Groovy,
    GwBasic,
    Haskell,
    Haxe,
    HicEst,
    HQ9Plus,
    HTML4STRICT,
    HTML5,
    Icon,
    IDL,
    INIfile,
    InnoScript,
    INTERCAL,
    IO,
    ISPFPanelDefinition,
    J,
    Java,
    Java5,
    JavaScript,
    JCL,
    jQuery,
    JSON,
    Julia,
    KiXtart,
    Kotlin,
    Latex,
    LDIF,
    LibertyBASIC,
    LindenScripting,
    Lisp,
    LLVM,
    LocoBasic,
    Logtalk,
    LOLCode,
    LotusFormulas,
    LotusScript,
    LScript,
    Lua,
    M68000Assembler,
    MagikSF,
    Make,
    MapBasic,
    Markdown,
    MatLab,
    mIRC,
    MIXAssembler,
    Modula2,
    Modula3,
    Motorola68000HiSoftDev,
    MPASM,
    MXML,
    MySQL,
    Nagios,
    NetRexx,
    newLISP,
    Nginx,
    Nimrod,
    None,
    NullSoftInstaller,
    Oberon2,
    ObjeckProgrammingLangua,
    ObjectiveC,
    OCalmBrief,
    OCaml,
    Octave,
    OpenObjectRexx,
    OpenBSDPACKETFILTER,
    OpenGLShading,
    OpenofficeBASIC,
    Oracle11,
    Oracle8,
    Oz,
    ParaSail,
    PARIGP,
    Pascal,
    Pawn,
    PCRE,
    Per,
    Perl,
    Perl6,
    PHP,
    PHPBrief,
    Pic16,
    Pike,
    PixelBender,
    PLI,
    PLSQL,
    PostgreSQL,
    PostScript,
    POVRay,
    PowerShell,
    PowerBuilder,
    ProFTPd,
    Progress,
    Prolog,
    Properties,
    ProvideX,
    Puppet,
    PureBasic,
    PyCon,
    Python,
    PythonforS60,
    QkdbPlus,
    QBasic,
    QML,
    R,
    Racket,
    Rails,
    RBScript,
    REBOL,
    REG,
    Rexx,
    Robots,
    RPMSpec,
    Ruby,
    RubyGnuplot,
    Rust,
    SAS,
    Scala,
    Scheme,
    Scilab,
    SCL,
    SdlBasic,
    Smalltalk,
    Smarty,
    SPARK,
    SPARQL,
    SQF,
    SQL,
    StandardML,
    StoneScript,
    SuperCollider,
    Swift,
    SystemVerilog,
    TSQL,
    TCL,
    TeraTerm,
    ThinBasic,
    TypoScript,
    Unicon,
    UnrealScript,
    UPC,
    Urbi,
    Vala,
    VBDotNET,
    VBScript,
    Vedit,
    VeriLog,
    VHDL,
    VIM,
    VisualProLog,
    VisualBasic,
    VisualFoxPro,
    WhiteSpace,
    WHOIS,
    Winbatch,
    XBasic,
    XML,
    XorgConfig,
    XPP,
    YAML,
    Z80Assembler,
    ZXBasic,
}

pub fn get_format(format: &Format) -> &str {
    match format {
        &Format::CS4 => "4cs",
        &Format::ACMECrossAssembler6502 => "6502acme",
        &Format::KickAssembler6502 => "6502kickass",
        &Format::TASM64TASS6502 => "6502tasm",
        &Format::ABAP => "abap",
        &Format::ActionScript => "actionscript",
        &Format::ActionScript3 => "actionscript3",
        &Format::Ada => "ada",
        &Format::AIMMS => "aimms",
        &Format::ALGOL68 => "algol68",
        &Format::ApacheLog => "apache",
        &Format::AppleScript => "applescript",
        &Format::APTSources => "apt_sources",
        &Format::ARM => "arm",
        &Format::ASMNASM => "asm",
        &Format::ASP => "asp",
        &Format::Asymptote => "asymptote",
        &Format::Autoconf => "autoconf",
        &Format::Autohotkey => "autohotkey",
        &Format::AutoIt => "autoit",
        &Format::Avisynth => "avisynth",
        &Format::Awk => "awk",
        &Format::BASCOMAVR => "bascomavr",
        &Format::Bash => "bash",
        &Format::Basic4GL => "basic4gl",
        &Format::Batch => "dos",
        &Format::BibTeX => "bibtex",
        &Format::BlitzBasic => "blitzbasic",
        &Format::Blitz3D => "b3d",
        &Format::BlitzMax => "bmx",
        &Format::BNF => "bnf",
        &Format::BOO => "boo",
        &Format::BrainFuck => "bf",
        &Format::C => "c",
        &Format::CWinAPI => "c_winapi",
        &Format::CMacs => "c_mac",
        &Format::CIntermediateLanguage => "cil",
        &Format::CSharp => "csharp",
        &Format::CPlusPlus => "cpp",
        &Format::CPlusPlusWinAPI => "cpp-winapi",
        &Format::CPlusPlusQt => "cpp-qt",
        &Format::CLoadrunner => "c_loadrunner",
        &Format::CADDCL => "caddcl",
        &Format::CADLisp => "cadlisp",
        &Format::Ceylon => "ceylon",
        &Format::CFDG => "cfdg",
        &Format::ChaiScript => "chaiscript",
        &Format::Chapel => "chapel",
        &Format::Clojure => "clojure",
        &Format::CloneC => "klonec",
        &Format::CloneCPlusPlus => "klonecpp",
        &Format::CMake => "cmake",
        &Format::COBOL => "cobol",
        &Format::CoffeeScript => "coffeescript",
        &Format::ColdFusion => "cfm",
        &Format::CSS => "css",
        &Format::Cuesheet => "cuesheet",
        &Format::D => "d",
        &Format::Dart => "dart",
        &Format::DCL => "dcl",
        &Format::DCPU16 => "dcpu16",
        &Format::DCS => "dcs",
        &Format::Delphi => "delphi",
        &Format::DelphiPrismOxygene => "oxygene",
        &Format::Diff => "diff",
        &Format::DIV => "div",
        &Format::DOT => "dot",
        &Format::E => "e",
        &Format::Easytrieve => "ezt",
        &Format::ECMAScript => "ecmascript",
        &Format::Eiffel => "eiffel",
        &Format::Email => "email",
        &Format::EPC => "epc",
        &Format::Erlang => "erlang",
        &Format::Euphoria => "euphoria",
        &Format::FSharp => "fsharp",
        &Format::Falcon => "falcon",
        &Format::Filemaker => "filemaker",
        &Format::FOLanguage => "fo",
        &Format::FormulaOne => "f1",
        &Format::Fortran => "fortran",
        &Format::FreeBasic => "freebasic",
        &Format::FreeSWITCH => "freeswitch",
        &Format::GAMBAS => "gambas",
        &Format::GameMaker => "gml",
        &Format::GDB => "gdb",
        &Format::Genero => "genero",
        &Format::Genie => "genie",
        &Format::GetText => "gettext",
        &Format::Go => "go",
        &Format::Groovy => "groovy",
        &Format::GwBasic => "gwbasic",
        &Format::Haskell => "haskell",
        &Format::Haxe => "haxe",
        &Format::HicEst => "hicest",
        &Format::HQ9Plus => "hq9plus",
        &Format::HTML4STRICT => "html4strict",
        &Format::HTML5 => "html5",
        &Format::Icon => "icon",
        &Format::IDL => "idl",
        &Format::INIfile => "ini",
        &Format::InnoScript => "inno",
        &Format::INTERCAL => "intercal",
        &Format::IO => "io",
        &Format::ISPFPanelDefinition => "ispfpanel",
        &Format::J => "j",
        &Format::Java => "java",
        &Format::Java5 => "java5",
        &Format::JavaScript => "javascript",
        &Format::JCL => "jcl",
        &Format::jQuery => "jquery",
        &Format::JSON => "json",
        &Format::Julia => "julia",
        &Format::KiXtart => "kixtart",
        &Format::Kotlin => "kotlin",
        &Format::Latex => "latex",
        &Format::LDIF => "ldif",
        &Format::LibertyBASIC => "lb",
        &Format::LindenScripting => "lsl2",
        &Format::Lisp => "lisp",
        &Format::LLVM => "llvm",
        &Format::LocoBasic => "locobasic",
        &Format::Logtalk => "logtalk",
        &Format::LOLCode => "lolcode",
        &Format::LotusFormulas => "lotusformulas",
        &Format::LotusScript => "lotusscript",
        &Format::LScript => "lscript",
        &Format::Lua => "lua",
        &Format::M68000Assembler => "m68k",
        &Format::MagikSF => "magiksf",
        &Format::Make => "make",
        &Format::MapBasic => "mapbasic",
        &Format::Markdown => "markdown",
        &Format::MatLab => "matlab",
        &Format::mIRC => "mirc",
        &Format::MIXAssembler => "mmix",
        &Format::Modula2 => "modula2",
        &Format::Modula3 => "modula3",
        &Format::Motorola68000HiSoftDev => "68000devpac",
        &Format::MPASM => "mpasm",
        &Format::MXML => "mxml",
        &Format::MySQL => "mysql",
        &Format::Nagios => "nagios",
        &Format::NetRexx => "netrexx",
        &Format::newLISP => "newlisp",
        &Format::Nginx => "nginx",
        &Format::Nimrod => "nimrod",
        &Format::None => "text",
        &Format::NullSoftInstaller => "nsis",
        &Format::Oberon2 => "oberon2",
        &Format::ObjeckProgrammingLangua => "objeck",
        &Format::ObjectiveC => "objc",
        &Format::OCalmBrief => "ocaml-brief",
        &Format::OCaml => "ocaml",
        &Format::Octave => "octave",
        &Format::OpenObjectRexx => "oorexx",
        &Format::OpenBSDPACKETFILTER => "pf",
        &Format::OpenGLShading => "glsl",
        &Format::OpenofficeBASIC => "oobas",
        &Format::Oracle11 => "oracle11",
        &Format::Oracle8 => "oracle8",
        &Format::Oz => "oz",
        &Format::ParaSail => "parasail",
        &Format::PARIGP => "parigp",
        &Format::Pascal => "pascal",
        &Format::Pawn => "pawn",
        &Format::PCRE => "pcre",
        &Format::Per => "per",
        &Format::Perl => "perl",
        &Format::Perl6 => "perl6",
        &Format::PHP => "php",
        &Format::PHPBrief => "php-brief",
        &Format::Pic16 => "pic16",
        &Format::Pike => "pike",
        &Format::PixelBender => "pixelbender",
        &Format::PLI => "pli",
        &Format::PLSQL => "plsql",
        &Format::PostgreSQL => "postgresql",
        &Format::PostScript => "postscript",
        &Format::POVRay => "povray",
        &Format::PowerShell => "powershell",
        &Format::PowerBuilder => "powerbuilder",
        &Format::ProFTPd => "proftpd",
        &Format::Progress => "progress",
        &Format::Prolog => "prolog",
        &Format::Properties => "properties",
        &Format::ProvideX => "providex",
        &Format::Puppet => "puppet",
        &Format::PureBasic => "purebasic",
        &Format::PyCon => "pycon",
        &Format::Python => "python",
        &Format::PythonforS60 => "pys60",
        &Format::QkdbPlus => "q",
        &Format::QBasic => "qbasic",
        &Format::QML => "qml",
        &Format::R => "rsplus",
        &Format::Racket => "racket",
        &Format::Rails => "rails",
        &Format::RBScript => "rbs",
        &Format::REBOL => "rebol",
        &Format::REG => "reg",
        &Format::Rexx => "rexx",
        &Format::Robots => "robots",
        &Format::RPMSpec => "rpmspec",
        &Format::Ruby => "ruby",
        &Format::RubyGnuplot => "gnuplot",
        &Format::Rust => "rust",
        &Format::SAS => "sas",
        &Format::Scala => "scala",
        &Format::Scheme => "scheme",
        &Format::Scilab => "scilab",
        &Format::SCL => "scl",
        &Format::SdlBasic => "sdlbasic",
        &Format::Smalltalk => "smalltalk",
        &Format::Smarty => "smarty",
        &Format::SPARK => "spark",
        &Format::SPARQL => "sparql",
        &Format::SQF => "sqf",
        &Format::SQL => "sql",
        &Format::StandardML => "standardml",
        &Format::StoneScript => "stonescript",
        &Format::SuperCollider => "sclang",
        &Format::Swift => "swift",
        &Format::SystemVerilog => "systemverilog",
        &Format::TSQL => "tsql",
        &Format::TCL => "tcl",
        &Format::TeraTerm => "teraterm",
        &Format::ThinBasic => "thinbasic",
        &Format::TypoScript => "typoscript",
        &Format::Unicon => "unicon",
        &Format::UnrealScript => "uscript",
        &Format::UPC => "upc",
        &Format::Urbi => "urbi",
        &Format::Vala => "vala",
        &Format::VBDotNET => "vbnet",
        &Format::VBScript => "vbscript",
        &Format::Vedit => "vedit",
        &Format::VeriLog => "verilog",
        &Format::VHDL => "vhdl",
        &Format::VIM => "vim",
        &Format::VisualProLog => "visualprolog",
        &Format::VisualBasic => "vb",
        &Format::VisualFoxPro => "visualfoxpro",
        &Format::WhiteSpace => "whitespace",
        &Format::WHOIS => "whois",
        &Format::Winbatch => "winbatch",
        &Format::XBasic => "xbasic",
        &Format::XML => "xml",
        &Format::XorgConfig => "xorg_conf",
        &Format::XPP => "xpp",
        &Format::YAML => "yaml",
        &Format::Z80Assembler => "z80",
        &Format::ZXBasic => "zxbasic",
    }
}