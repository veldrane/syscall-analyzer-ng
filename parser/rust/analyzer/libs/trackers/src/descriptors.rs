use std::ops::DerefMut;
use uuid::Uuid;
use std::ops::Deref;

#[derive(Debug, Clone)]
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

    pub fn new_default_fd(created: f64, fd_type: DefaultFd) -> Self {
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

#[derive(Debug, Clone)]
pub struct Descs(Vec<DescRecord>);


impl Descs {
    
    pub fn new() -> Self {
        Descs(Vec::new())
    }

    pub fn init_empty_process(created: f64) -> Self {
        let mut fds = Descs::new();

        fds.0.push(DescRecord::new_default_fd(created, DefaultFd::Stdin));
        fds.0.push(DescRecord::new_default_fd(created, DefaultFd::Stdout));
        fds.0.push(DescRecord::new_default_fd(created, DefaultFd::Stderr));
        fds
    }

    pub fn is_present(&self, d: &DescRecord) -> bool {
        self.iter().any(|f| (f.fd == d.fd) && (f.deleted == false))
    }

    pub fn get_by_descriptor_number(&self, d: i32) -> Option<&DescRecord> {
        self.iter().find(|r| (r.fd == d) && (r.deleted == false))
    }

    pub fn get_mut_by_descriptor_number(&mut self, d: i32) -> Option<&mut DescRecord> {
        self.iter_mut().find(|r| (r.fd == d) && (r.deleted == false))
    }

    pub fn get_mut_by_uuid(&mut self, uuid: &str) -> Option<&mut DescRecord> {
            self.iter_mut().find(|r| (r.uuid == uuid) && (r.deleted == false))
        }

    pub fn add(&mut self, created: f64, d: i32, path: String, d_type: DescType) -> Result<String, FdsError> {

        let desc_record = DescRecord::new(created, d, path, d_type);
        
        //if self.is_present(&desc_record) {
        //    return Err(FdsError::FdAlreadyExists);
        //}

        let uuid = desc_record.uuid.clone();
        self.0.push(desc_record);
        Ok(uuid)
    }
    
    pub fn close(&mut self, uuid: &str, closed: f64) -> Result<(), FdsError> {

        let desc_record = self.get_mut_by_uuid(uuid).ok_or(FdsError::FdNotFound)?;

        desc_record.closed = Some(closed);
        desc_record.deleted = true;
        Ok(())
    }

    pub fn close_by_descriptor_number(&mut self, d: i32, closed: f64) -> Result<(), FdsError> {

        let desc_record = self.get_mut_by_descriptor_number(d).ok_or(FdsError::FdNotFound)?;

        desc_record.closed = Some(closed);
        desc_record.deleted = true;
        Ok(())
    }

    pub fn del_by_descriptor_number(&mut self, d: i32) -> Result<(), FdsError> {

        let desc_record = self.get_by_descriptor_number(d).ok_or(FdsError::FdNotFound)?;
        // desc_record.deleted = true;
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