fn main() {
    println!("Hello, world!");
}

pub struct Build {
    pub target: String,
}

impl Default for Build {
    fn default() -> Self {
        Self {
            target: String::from("target"),
        }
    }
}

pub struct Error;

pub struct Package;

pub struct Publish;

pub enum Arch {
    Amd64,
    I386,
    Aarch64(Arm),
    Aarch32,
}

pub enum Arm {
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
    fn build(&self, _: String, os: OS, _: Arch) -> Result<Build, Error> {
        let _os = match os {
            OS::Linux => "GOOS=linux",
            OS::Windows => "GOOS=windows",
            OS::Darwin => "GOOS=darwin",
            OS::FreeBSD => "GOOS=freebsd",
            OS::OpenBSD => "GOOS=openbsd",
            OS::NetBSD => "GOOS=netbsd",
        };

        println!("{}", _os);

        Ok(Build::default())
    }
}
