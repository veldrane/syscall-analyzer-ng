use tokio::sync::mpsc;
use tokio::runtime::Builder;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use std::fs::File as StdFile;
use std::path::PathBuf;



pub struct DocumentSaver {
    tx: mpsc::Sender<Document>,
    _saver: tokio::task::JoinHandle<()>,
    _rt: tokio::runtime::Runtime,
}

struct Document(String);

impl DocumentSaver {


    pub fn build() -> Self {
        

        let (tx, rx): (mpsc::Sender<Document>, mpsc::Receiver<Document>) = mpsc::channel(1024);

        let path = PathBuf::from("/tmp/document_saver.json");
        let file = StdFile::create(&path).expect("Cannot open log file");

        let rt = Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("logger")
        .enable_all()
        .build().expect("Failed to create runtime");
    
        let document_saver: tokio::task::JoinHandle<_> = rt.block_on(async {
            let tokio_file = TokioFile::from_std(file);
            tokio::spawn(doc_receiver(rx, tokio_file))
        });


        let document_saver = DocumentSaver {
            tx,
            _saver: document_saver,
            _rt: rt,
        };

        return document_saver;
    }


    pub fn send(&self, doc: String) {
        let _ = self.tx.blocking_send(Document(doc));
    }

}




async fn doc_receiver(mut rx: mpsc::Receiver<Document>, mut file: TokioFile) {
    while let Some(Document(line)) = rx.recv().await {
        let full_line = format!("{}\n", line);
        if let Err(e) = file.write_all(full_line.as_bytes()).await {
            eprintln!("Failed to write to log file: {}", e);
        }
    }
}