fn main() {
    println!("Hello, world!");

    let builder = GoBuilder {
        file: String::from("cmd/foo/main.go"),
        _before_build: Some(String::from("Hello world")),
        _after_build: None,
    };

    match builder.build(
        String::from("linux"),
        String::from("bin/foo"),
        OS::OpenBSD,
        Arch::Arm64(Arm64::V6),
    ) {
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
    fn before_build(&self) {
        println!("before build")
    }

    fn build(&self, id: String, target: String, os: OS, arch: Arch) -> Result<Build, Error>;

    fn after_build(&self) {
        println!("after build")
    }
}

pub struct GoBuilder {
    pub file: String,
    pub _before_build: Option<String>,
    pub _after_build: Option<String>,
}

impl Builder for GoBuilder {
    fn build(&self, _: String, target: String, os: OS, arch: Arch) -> Result<Build, Error> {
        if let Some(s) = &self._before_build {
            println!("{}", s);
            self.before_build();
        }

        const LINUX: &'static str = "linux";
        const WINDOWS: &'static str = "windows";
        const DARWIN: &'static str = "darwin";
        const FREEBSD: &'static str = "freebsd";
        const OPENBSD: &'static str = "openbsd";
        const NETBSD: &'static str = "netbsd";

        let os_str = match os {
            OS::Linux => LINUX,
            OS::Windows => WINDOWS,
            OS::Darwin => DARWIN,
            OS::FreeBSD => FREEBSD,
            OS::OpenBSD => OPENBSD,
            OS::NetBSD => NETBSD,
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

        let mut target = format!("{}_{}_{}", target, os_str, arch_str);

        if let OS::Windows = os {
            target = format!("{}.exe", target,)
        }

        if let Some(_) = self._after_build {
            self.after_build()
        }

        Ok(Build {
            command: format!(
                "GOOS={} GOARCH={} go build -o {} {}",
                os_str, arch_str, target, self.file
            ),
        })
    }
}
