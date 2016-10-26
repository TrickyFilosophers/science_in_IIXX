use event::{Event, Operation};
use git2::Repository;
use git2::Error;
use git2::Signature;
use git2::IndexAddOption;
use git2::Time;

const TEXT : &'static str = 
"░░░░░░░░░▄░░░░░░░░░░░░░░▄
░░░░░░░░▌▒█░░░░░░░░░░░▄▀▒▌
░░░░░░░░▌▒▒█░░░░░░░░▄▀▒▒▒▐
░░░░░░░▐▄▀▒▒▀▀▀▀▄▄▄▀▒▒▒▒▒▐
░░░░░▄▄▀▒░▒▒▒▒▒▒▒▒▒█▒▒▄█▒▐
░░░▄▀▒▒▒░░░▒▒▒░░░▒▒▒▀██▀▒▌
░░▐▒▒▒▄▄▒▒▒▒░░░▒▒▒▒▒▒▒▀▄▒▒▌
░░▌░░▌█▀▒▒▒▒▒▄▀█▄▒▒▒▒▒▒▒█▒▐
░▐░░░▒▒▒▒▒▒▒▒▌██▀▒▒░░░▒▒▒▀▄▌
░▌░▒▄██▄▒▒▒▒▒▒▒▒▒░░░░░░▒▒▒▒▌
▌▒▀▐▄█▄█▌▄░▀▒▒░░░░░░░░░░▒▒▒▐
▐▒▒▐▀▐▀▒░▄▄▒▄▒▒▒▒▒▒░▒░▒░▒▒▒▒▌
▐▒▒▒▀▀▄▄▒▒▒▄▒▒▒▒▒▒▒▒░▒░▒░▒▒▐
░▌▒▒▒▒▒▒▀▀▀▒▒▒▒▒▒░▒░▒░▒░▒▒▒▌
░▐▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒░▒░▒▒▄▒▒▐
░░▀▄▒▒▒▒▒▒▒▒▒▒▒░▒░▒░▒▄▒▒▒▒▌
░░░░▀▄▒▒▒▒▒▒▒▒▒▒▄▄▄▀▒▒▒▒▄▀
░░░░░░▀▄▄▄▄▄▄▀▀▀▒▒▒▒▒▄▄▀
░░░░░░░░░▒▒▒▒▒▒▒▒▒▒▀▀
 
So academic. Much knowledge. Wow.


";

use std::path::{PathBuf, Path};
use std::fs;
use std::io::Write;

pub struct Evaluator {
    repo: Repository,
    path: String,
    secs: i64,
}

impl Evaluator {
    pub fn new(repo_path: String) -> Evaluator {
        Evaluator {
            repo: Repository::open(&repo_path).unwrap(),
            path: repo_path,
            secs: 0
        }
    }

    pub fn evaluate<I>(&mut self, events: I)
        where I: IntoIterator<Item=Event> 
    {
        for event in events {
            println!("{:?}: {:?}", event.operation, event.path);
            for _ in 0..event.repeat.unwrap_or(1) {
                match event.operation {
                    Operation::Pause => self.secs += 3600 * 10,
                    Operation::CreateDir => fs::create_dir_all(self.build_path(&event)).unwrap(),
                    Operation::AppendFile => self.append(&event),
                    Operation::AppendMany => self.append_many(&event),
                    Operation::DeleteFile => self.delete(&event),
                    Operation::DeleteMany => self.delete_many(&event),
                    Operation::Commit => self.commit(&event)
                };
                self.secs += 600;    
            }
            self.secs += 3600;
        }
    }

    fn build_path(&self, event: &Event) -> PathBuf {
        let mut pathbuf = PathBuf::from(&self.path);
        pathbuf.push(Path::new(event.path.as_ref().unwrap()));
        pathbuf
    }

    fn append_many(&self, event: &Event) {
        let from = event.from.unwrap_or(0);
        let to = event.to.unwrap();

        for i in from..to {
            let param_path = event.path.as_ref().unwrap();
            let param_path = param_path.replace("{}", &format!("{}", i));
            
            let mut pathbuf = PathBuf::from(&self.path);
            pathbuf.push(Path::new(&param_path));

            self.append_file(&pathbuf);
        }
    }
    
    fn delete_many(&self, event: &Event) {
        let from = event.from.unwrap_or(0);
        let to = event.to.unwrap();

        for i in from..to {
            let param_path = event.path.as_ref().unwrap();
            let param_path = param_path.replace("{}", &format!("{}", i));
            
            let mut pathbuf = PathBuf::from(&self.path);
            pathbuf.push(Path::new(&param_path));

            self.delete_file(&pathbuf);
        }
    }

    fn append(&self, event: &Event) {
        let path = self.build_path(event);
        self.append_file(&path);
    }

    fn delete(&self, event: &Event) {
        let path = self.build_path(event);
        self.delete_file(&path);
    }

    fn append_file(&self, path: &Path) {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .unwrap();
        
        writeln!(file, "{}", TEXT);
    }
    
    fn delete_file(&self, path: &Path) {
        fs::remove_file(path).unwrap();
    }

    fn add_all(&self, event: &Event) {
        let mut index = self.repo.index().unwrap();
        index.add_all(&["."], IndexAddOption::all(), None).unwrap();
        index.write().unwrap()
    }

    fn commit(&self, event: &Event) {
        self.add_all(event);

        let tree_id = {
            let mut index = self.repo.index().unwrap();
            index.write_tree().unwrap()
        };

        let tree = self.repo.find_tree(tree_id).unwrap();
        let parent = self.repo.refname_to_id("HEAD").and_then(|id| {
            self.repo.find_commit(id)
        });

        let time = Time::new(self.secs, 0);
        let name = event.name.as_ref().unwrap();
        let mut email = name.clone(); email.push_str("@empire.ru");

        if let Ok(parent) = parent {
            self.repo.commit(
                Some("HEAD"),
                &Signature::new(name, &email, &time).unwrap(),
                &Signature::new(name, &email, &time).unwrap(),
                event.msg.as_ref().unwrap(),
                &tree,
                &[&parent]
            ).unwrap();
        } else {
            self.repo.commit(
                Some("HEAD"),
                &Signature::new(name, &email, &time).unwrap(),
                &Signature::new(name, &email, &time).unwrap(),
                event.msg.as_ref().unwrap(),
                &tree,
                &[]
            ).unwrap();
        }    
    }
}

