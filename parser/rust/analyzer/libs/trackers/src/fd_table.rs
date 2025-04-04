use std::ops::DerefMut;
use uuid::Uuid;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum DescType {
    File,
    Socket,
    Epoll,
    Time,
    Event,
    Pipe,
}

impl FromStr for DescType {

    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s {
            s if s.contains("socket:") => {
                Ok(DescType::Socket)
            },
            s if s.contains("anon_inode:[timerfd]") => {
                Ok(DescType::Time)
            },
            s if s.contains("anon_inode:[eventfd]") => {
                Ok(DescType::Event)
            },
            s if s.contains("anon_inode:[epoll]") => {
                Ok(DescType::Epoll)
            },
            _ => {
                Ok(DescType::File)
            }
        }
    }
}

pub enum StdFd {
    Stdin,
    Stdout,
    Stderr,
}

pub enum FdsError {
    FdNotFound,
    FdAlreadyExists,
}


#[derive(Debug, Clone)]
pub struct DescRecord {
    pub created: f64,
    pub closed: Option<f64>,
    pub uuid: String,
    pub fd: i32,
    pub path: String,
    pub fd_type: DescType,
    pub deleted: bool,
}

impl DescRecord {

    pub fn new(created: f64, fd: i32, path: String, fd_type: DescType) -> Self {
        Self {
            created: created,
            closed: None,
            uuid: Uuid::new_v4().to_string(),
            path: path,
            fd_type: fd_type,
            deleted: false,
            fd: fd,
        }
    }

    pub fn std_fd(created: f64, fd_type: StdFd) -> Self {
        let (path, fd) = match fd_type {
            StdFd::Stdin => { ("/dev/stdin", 0) },
            StdFd::Stdout => { ("/dev/stdout", 1) },
            StdFd::Stderr => { ("/dev/stderr", 2) },
        };

        Self {
            created: created,
            closed: None,
            uuid: Uuid::new_v4().to_string(),
            fd:fd,
            path: path.to_string(),
            fd_type: DescType::File,
            deleted: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Descs(Vec<DescRecord>);


impl Descs {
    pub fn new() -> Self {
        Descs(Vec::new())
    }

    pub fn with_std_fds(created: f64) -> Self {
        let mut fds = Descs::new();
        fds.0.push(DescRecord::std_fd(created, StdFd::Stdin));
        fds.0.push(DescRecord::std_fd(created, StdFd::Stdout));
        fds.0.push(DescRecord::std_fd(created, StdFd::Stderr));
        fds
    }

    pub fn contains(&self, d: &DescRecord) -> bool {
        self.iter().any(|f| (f.fd == d.fd) && !f.deleted)
    }

    pub fn get_fd(&self, fd: i32) -> Option<&DescRecord> {
        self.iter().find(|r| (r.fd == fd) && !r.deleted)
    }

    pub fn get_fd_mut(&mut self, fd: i32) -> Option<&mut DescRecord> {
        self.iter_mut().find(|r| (r.fd == fd) && !r.deleted)
    }

    pub fn find_by_uuid_mut(&mut self, uuid: &str) -> Option<&mut DescRecord> {
        self.iter_mut().find(|r| (r.uuid == uuid) && !r.deleted)
    }

    pub fn add(&mut self, created: f64, fd: i32, path: String, d_type: DescType) -> Result<String, FdsError> {
        let desc_record = DescRecord::new(created, fd, path, d_type);
        let uuid = desc_record.uuid.clone();
        self.0.push(desc_record);
        Ok(uuid)
    }

    pub fn close(&mut self, uuid: &str, closed: f64) -> Result<(), FdsError> {
        let desc_record = self.find_by_uuid_mut(uuid).ok_or(FdsError::FdNotFound)?;
        desc_record.closed = Some(closed);
        desc_record.deleted = true;
        Ok(())
    }

    pub fn close_fd(&mut self, fd: i32, closed: f64) -> Result<(), FdsError> {
        let desc_record = self.get_fd_mut(fd).ok_or(FdsError::FdNotFound)?;
        desc_record.closed = Some(closed);
        desc_record.deleted = true;
        Ok(())
    }

    pub fn close_range(&mut self, min_fd: i32, max_fd: i32, closed: f64) -> Result<(), FdsError> {
        for desc_record in self.iter_mut() {
            if desc_record.fd >= min_fd && desc_record.fd <= max_fd {
                desc_record.closed = Some(closed);
                desc_record.deleted = true;
            }
        }
        Ok(())
    }

    pub fn remove_fd(&mut self, fd: i32) -> Result<(), FdsError> {
        // Příklad: zde by mohlo dojít k odstranění nebo označení záznamu jako smazaného.
        let desc_record = self.get_fd_mut(fd).ok_or(FdsError::FdNotFound)?;
        // Např. můžeme nastavit deleted na true:
        desc_record.deleted = true;
        Ok(())
    }
}


impl Deref for Descs {
    type Target = Vec<DescRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Descs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}