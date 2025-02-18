use std::collections::HashMap;
use std::fmt::Debug;

type ParserFn = Box<dyn Fn(&str) -> Result<Box<dyn SyscallArgs>, String> + Send + Sync>;

trait SyscallArgs: 'static + Debug {
    fn parse(input: &str) -> Result<Self, String>
    where
        Self: Sized;

    fn register(registry: &mut HashMap<String, ParserFn>, name: &'static str)
    where
        Self: Sized,
    {
        registry.insert(
            name.to_string(),
            Box::new(|input: &str| {
                Self::parse(input)
                    .map(|parsed| Box::new(parsed) as Box<dyn SyscallArgs>)
            }),
        );
    }
}

#[derive(Debug)]
struct DefaultSyscallArgs {
    raw: String,
}

#[derive(Debug)]
struct Syscall {
    time: String,
    name: String,
    args: Option<Box<dyn SyscallArgs>>,
}


impl SyscallArgs for DefaultSyscallArgs {
    fn parse(input: &str) -> Result<Self, String> {
        Ok(DefaultSyscallArgs {
            raw: input.to_string(),
        })
    }
}

#[derive(Debug)]
struct MmapArgs {
    // Zde můžete mít specifická pole, například adresa, velikost atd.
    file_name: String,
    fd: i32,
}

#[derive(Debug)]
struct OpenArgs {
    flags: i32,
}


impl SyscallArgs for OpenArgs {
    fn parse(input: &str) -> Result<Self, String> {
        Ok(OpenArgs { flags: 123 })
    }
    
}

impl SyscallArgs for MmapArgs {
    fn parse(input: &str) -> Result<Self, String> {
        // Například zde ještě nedokončená implementace.
        Ok(MmapArgs {
            file_name: "test.txt".to_string(),
            fd: 123,
        })
        //Err("Parsing pro mmap není implementován.".into())
    }
}

fn main() {
    let mut registry: HashMap<String, ParserFn> = HashMap::new();

    // Registrace parseru pro "mmap" s explicitním jménem
    MmapArgs::register(&mut registry, "mmap");
    OpenArgs::register(&mut registry, "open");

    let line = "123.456 open arg1, arg2, arg3";
    let parts: Vec<&str> = line.splitn(2, " ").collect();
    let mut syscall: Syscall = Syscall {
        time: parts[0].to_string(),
        name: parts[1].split_whitespace().next().unwrap().to_string(),
        args: None,
    };
    let args = parts[1].trim_start_matches(syscall.name.as_str()).trim();

    let result = if let Some(parser) = registry.get(syscall.name.as_str()) {
        parser(args)
    } else {
        DefaultSyscallArgs::parse(args)
            .map(|v| Box::new(v) as Box<dyn SyscallArgs>)
    };

    match result {
        Ok(parsed_args) => {
            println!("Syscall {} byl úspěšně parsován.", syscall.name.as_str());
            // Výpis detailů pomocí formátování Debug
            syscall.args = Some(parsed_args);
            println!("Parsed syscall (debug): {:?}", syscall);
        },
        Err(e) => {
            eprintln!("Chyba při parsování syscallu {}: {}", syscall.name.as_str(), e);
        },
    }
}
