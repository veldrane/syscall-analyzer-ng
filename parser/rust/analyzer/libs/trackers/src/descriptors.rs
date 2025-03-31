use std::{ops::DerefMut, time::{Duration, SystemTime}};
use uuid::Uuid;
use std::ops::Deref;


pub enum DescType {
    File,
    Socket,
    Epoll,
    Unknown,
}

pub enum DefaultFd {
    Stdin,
    Stdout,
    Stderr,
}

pub enum FdsError {
    FdNotFound,
    FdAlreadyExists,
}


pub struct DescRecord {
    pub created: String,
    pub closed: Option<String>,
    pub uuid: String,
    pub fd: i32,
    pub path: String,
    pub fd_type: DescType,
    pub deleted: bool,
}

impl DescRecord {

    pub fn new(created: String, fd: i32, path: String, fd_type: DescType) -> Self {
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

    pub fn new_default_fd(created: String, fd_type: DefaultFd) -> Self {
        let (path, fd) = match fd_type {
            DefaultFd::Stdin => { ("/dev/stdin", 0) },
            DefaultFd::Stdout => { ("/dev/stdout", 1) },
            DefaultFd::Stderr => { ("/dev/stderr", 2) },
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

pub struct Descs(Vec<DescRecord>);


impl Descs {
    
    pub fn new() -> Self {
        Descs(Vec::new())
    }

    pub fn init_empty_process(created: String) -> Self {
        let mut fds = Descs::new();

        fds.0.push(DescRecord::new_default_fd(created.clone(), DefaultFd::Stdin));
        fds.0.push(DescRecord::new_default_fd(created.clone(), DefaultFd::Stdout));
        fds.0.push(DescRecord::new_default_fd(created.clone(), DefaultFd::Stderr));
        fds
    }

    pub fn is_present(&self, d: &DescRecord) -> bool {
        self.iter().any(|f| (f.fd == d.fd) && (f.deleted == false))
    }

    pub fn get_by_d(&self, d: i32) -> Option<&DescRecord> {
        self.iter().find(|r| r.fd == d)
    }

    pub fn get_mut_by_uuid(&mut self, uuid: &str) -> Option<&mut DescRecord> {
            self.iter_mut().find(|r| r.uuid == uuid)
        }

    pub fn add(&mut self, created: String, d: i32, path: String, d_type: DescType) -> Result<String, FdsError> {

        let desc_record = DescRecord::new(created, d, path, d_type);
        
        //if self.is_present(&desc_record) {
        //    return Err(FdsError::FdAlreadyExists);
        //}

        let uuid = desc_record.uuid.clone();
        self.0.push(desc_record);
        Ok(uuid)
    }
    
    pub fn close(&mut self, uuid: &str, closed: String) -> Result<(), FdsError> {

        let desc_record = self.get_mut_by_uuid(uuid).ok_or(FdsError::FdNotFound)?;

        desc_record.closed = Some(closed);
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