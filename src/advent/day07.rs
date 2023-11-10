use super::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Cmd {
    cd: String,
    ls: Vec<String>,
}

pub struct Day07 {}
impl Puzzle for Day07 {
    fn part_one(&self, data: &'static str) -> String {
        let commands = parse(data);
        let space = disk_space(&commands);

       
        let answer = space.values().filter(|&&x| x <= 100000).sum::<usize>();
        answer.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let commands = parse(data);
        let space = disk_space(&commands);
        
        let delete = 30000000 - (70000000 - space["//"]);
        let answer = space.values().filter(|&&x| x >= delete).min().unwrap();
        answer.to_string()
    }
}

fn parse(data: &str) -> Vec<Cmd> {
    let mut path = "".to_string();
    data.lines().fold(vec![], |mut cmds, line| {
        if line.starts_with("$ cd") {
            let arg = line.strip_prefix("$ cd ").unwrap();

            if arg == ".." {
                let (parent, _) = path.strip_suffix('/').unwrap().rsplit_once('/').unwrap();
                path = format!("{}/", parent);
            } else {
                path = format!("{}{}/", path, arg);
                cmds.push(Cmd{cd: path.clone(), ls: vec![]});
            }
        } else if !line.starts_with('$') {
            let prev = cmds.last_mut().unwrap();

            if line.starts_with("dir") {
                let arg = line.strip_prefix("dir ").unwrap();
                prev.ls.push(format!("{}{}/", path, arg));
            } else {
                let (arg, _) = line.split_once(' ').unwrap();
                prev.ls.push(arg.to_string());
            }
        }
        cmds
    })
}

fn disk_space(cmds: &[Cmd]) -> HashMap<&str, usize> {
    cmds.iter().rev().fold(HashMap::<&str, usize>::new(), |mut map, cmd| {
        let size = cmd.ls.iter().map(|x| {
            if x.starts_with('/') { 
                map[x.as_str()] 
            } else { 
                x.parse().unwrap() 
            }
        }).sum();
        map.insert(cmd.cd.as_str(), size);
        map
    })
}
