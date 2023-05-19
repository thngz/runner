use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Output;

#[derive(Serialize, Debug)]
struct PistonFile {
    name: String,
    content: String,
}
#[derive(Serialize, Debug)]
struct PistonInput {
    language: String,
    version: String,
    files: [PistonFile; 1],
    stdin: String,
}

#[derive(Deserialize, Debug)]
struct PistonRunResult {
    stdout: String,
    stderr: String,
    code: i32,
    signal: Option<String>,
    output: String,
}

#[derive(Deserialize)]
struct PistonOutput {
    language: String,
    version: String,
    run: PistonRunResult,
}

#[derive(Deserialize)]
struct Module {
    exercise: Vec<Exercise>,
}

#[derive(Deserialize)]
struct Exercise {
    name: String,
    test: Vec<Test>,
}

#[derive(Deserialize)]
struct Test {
    test_name: String,
    input: Vec<String>,
    output: Vec<String>,
}

#[tokio::main]
async fn main() {
    let endpoint = "https://emkc.org/api/v2/piston/execute";
    let client = reqwest::Client::new();

    let path = get_path();
    let files = get_files(&path);
    let module = parse_rules(&path);

    for exercise in module.exercise {
        let exercise_files: Vec<&PistonFile> = files
            .iter()
            .filter(|file| file.name == exercise.name)
            .collect();

        let exercise_file = exercise_files.first().unwrap();

        for test in exercise.test {
            for input in test.input {
                let payload = PistonInput {
                    language: String::from("javascript"),
                    version: String::from("1.32.3"),
                    files: [PistonFile {
                        name: exercise_file.name.clone(),
                        content: exercise_file.content.clone(),
                    }],
                    stdin: String::from(input),
                };

                let resp = match client.post(endpoint).json(&payload).send().await {
                    Ok(resp) => resp.text().await.unwrap(),
                    Err(err) => panic!("something went wrong: {}", err),
                };

                let result: PistonOutput = serde_json::from_str(resp.as_str()).unwrap();
                let piston_output = result.run.output;
                for output in test.output.clone() {
                    //println!("{} vs {}", piston_output, output);
                    if piston_output.trim() == output.trim() {
                        println!("{} passed ✅", test.test_name);
                    } else {
                        println!("{} failed ❌", test.test_name);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(205));
            }
        }
    }
}

fn parse_rules(path: &PathBuf) -> Module {
    let rules = fs::read_to_string(path.join("rules.toml")).expect("Failed to read TOML");
    let module: Module = toml::from_str(&rules).unwrap();

    return module;
}

fn get_files(path: &PathBuf) -> Vec<PistonFile> {
    let mut piston_files: Vec<PistonFile> = vec![];
    let files = path.read_dir().unwrap();

    for file in files {
        if file.is_err() {
            panic!("Cant read file");
        }

        let file = file.unwrap();

        let file_name = file.file_name().to_str().unwrap().to_string();

        let file_with_path = path.join(&file_name).to_str().unwrap().to_string();

        let piston_file = PistonFile {
            name: file_name.to_string(),
            content: fs::read_to_string(file_with_path).unwrap(),
        };

        piston_files.push(piston_file);
    }

    piston_files
}

fn get_path() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    let path_string = args.get(1).unwrap().to_string();
    return PathBuf::from(&path_string);
}
