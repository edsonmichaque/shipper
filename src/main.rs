fn main() {
    let builder = GoBuilder {};

    let opts = BuildOptions {
        work_dir: None,
        id: String::from("linux"),
        os: OS::Windows,
        arch: Arch::Arm64(Arm64::V7),
        target: String::from("foo"),
        setup: None,
        teardown: None,
        files: Some(Vec::from([String::from("cmd/foo/main.go")])),
    };

    match builder.build(&opts) {
        Ok(build) => println!("{:?}", build),
        Err(_) => println!("boom"),
    };
}

#[derive(Debug)]
pub struct Build {
    pub command: String,
    pub target: String,
}

pub struct Error;

#[derive(Debug)]
pub enum Arch {
    Amd64,
    X86,
    Arm64(Arm64),
    Arm,
}

#[derive(Debug)]
pub enum Arm64 {
    V6,
    V7,
}

#[derive(Debug)]
pub enum OS {
    Linux,
    Windows,
    Darwin,
    FreeBSD,
    OpenBSD,
    NetBSD,
}

pub trait Builder {
    fn build(&self, opts: &BuildOptions) -> Result<Build, Error>;
}

const LINUX: &'static str = "linux";
const WINDOWS: &'static str = "windows";
const DARWIN: &'static str = "darwin";
const FREEBSD: &'static str = "freebsd";
const OPENBSD: &'static str = "openbsd";
const NETBSD: &'static str = "netbsd";

pub struct GoBuilder {}

#[derive(Debug)]
pub struct BuildOptions {
    pub work_dir: Option<String>,
    pub id: String,
    pub target: String,
    pub os: OS,
    pub arch: Arch,
    pub setup: Option<String>,
    pub teardown: Option<String>,
    pub files: Option<Vec<String>>,
}

impl Builder for GoBuilder {
    fn build(&self, opts: &BuildOptions) -> Result<Build, Error> {
        if let Some(s) = &opts.setup {
            println!("{}", s);
        }

        let build_os = match opts.os {
            OS::Linux => LINUX,
            OS::Windows => WINDOWS,
            OS::Darwin => DARWIN,
            OS::FreeBSD => FREEBSD,
            OS::OpenBSD => OPENBSD,
            OS::NetBSD => NETBSD,
        };

        let build_arch = match &opts.arch {
            Arch::Amd64 => "amd64",
            Arch::X86 => "386",
            Arch::Arm => "arm",
            Arch::Arm64(_) => "arm64",
        };

        let mut build_target = format!("{}_{}_{}", opts.target, build_os, build_arch);

        if let OS::Windows = opts.os {
            build_target = format!("{}.exe", build_target)
        }

        if let Some(s) = &opts.teardown {
            println!("{}", s);
        }

        let mut build_file = String::from("main.go");

        if let Some(files) = &opts.files {
            if files.len() != 0 {
                build_file = String::from("");
            }

            for file in files {
                build_file = format!("{} {}", build_file, file)
            }
        }

        Ok(Build {
            command: format!(
                "GOOS={} GOARCH={} go build -o {} {}",
                build_os, build_arch, build_target, build_file
            ),
            target: build_target,
        })
    }
}
