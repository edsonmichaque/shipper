fn main() {
    println!("Hello, world!");

    let builder = GoBuilder {
        file: String::from("cmd/foo/main.go"),
    };

    match builder.build(String::from("linux"), OS::NetBSD, Arch::Arm64(Arm64::V6)) {
        Ok(build) => println!("{:?}", build),
        Err(_) => println!("boom"),
    };
}

#[derive(Debug)]
pub struct Build {
    pub command: String,
}

pub struct Error;

pub struct Package;

pub struct Publish;

pub enum Arch {
    Amd64,
    X86,
    Arm64(Arm64),
    Arm,
}

pub enum Arm64 {
    V6,
    V7,
}

pub enum OS {
    Linux,
    Windows,
    Darwin,
    FreeBSD,
    OpenBSD,
    NetBSD,
}

pub trait Builder {
    fn build(&self, id: String, os: OS, arch: Arch) -> Result<Build, Error>;
}

pub struct GoBuilder {
    pub file: String,
}

impl Builder for GoBuilder {
    fn build(&self, _: String, os: OS, arch: Arch) -> Result<Build, Error> {
        let os_str = match os {
            OS::Linux => "linux",
            OS::Windows => "windows",
            OS::Darwin => "darwin",
            OS::FreeBSD => "freebsd",
            OS::OpenBSD => "openbsd",
            OS::NetBSD => return Err(Error),
        };

        let arch_str = match arch {
            Arch::Amd64 => "amd64",
            Arch::X86 => "386",
            Arch::Arm => "arm",
            Arch::Arm64(v) => match v {
                Arm64::V6 => "arm64",
                Arm64::V7 => "arm64",
            },
        };

        Ok(Build {
            command: format!("GOOS={} GOARCH={} go build {}", os_str, arch_str, self.file),
        })
    }
}
