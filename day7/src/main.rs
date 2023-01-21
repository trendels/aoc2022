// This problem took way longer than all of the previous ones, because I had to learn so much about
// the rust borrow checker. Getting the tree data structure right was surprisingly complicated.
//
// Here is what I learned:
//
// - You can't have a tree where the nodes have children as well as a reference back to the parent.
// - One of the ways to work around this is to store references to the nodes in an external data
//   structure, like a HashMap.
// - You need `Rc` (reference counted pointers) if you want to store data in more than one
//   place (e.g. nodes are stored in the tree as well as in the hash map).
// - You can not get a value "out" of an `Rc`!
// - `Rc`s are always immutable. If you want to mutate the wrapped object you need `RefCell`.
// - `RefCell` allows to mutate an object even while immutable references exist. Wrapping an
//   object in a `RefCell` means the ownership rules are checked at runtime instead of at compile
//   time: Violating them will cause a runtime error.
//
// There are still some uses of `unwrap()` in functions other than `main`, which I'd like to avoid.
//
// I had to convert between `&str` and `String` a lot (`.as_str()`, `.to_string()`). I wonder if
// there is a better way to do this.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(String, u64),
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn read_input(input: &str) -> Option<Vec<Line>> {
    let mut result = Vec::new();
    for line in input.lines() {
        if let Some(line) = line.strip_prefix("$ ") {
            let cmd = parse_command(line)?;
            result.push(Line::Command(cmd));
        } else {
            let entry = parse_entry(line)?;
            result.push(Line::Entry(entry));
        }
    }
    Some(result)
}

fn parse_command(line: &str) -> Option<Command> {
    if line.starts_with("ls") {
        Some(Command::Ls)
    } else if let Some(name) = line.strip_prefix("cd ") {
        Some(Command::Cd(name.to_string()))
    } else {
        None
    }
}

fn parse_entry(line: &str) -> Option<Entry> {
    if let Some(name) = line.strip_prefix("dir ") {
        Some(Entry::Dir(name.to_string()))
    } else {
        let (size, name) = line.split_once(' ')?;
        Some(Entry::File(name.to_string(), size.parse().ok()?))
    }
}

#[derive(Debug)]
struct Node {
    size: u64,
    children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
struct Tree {
    root: Rc<Node>,
    lookup: HashMap<String, Rc<Node>>,
}

fn build_tree(lines: Vec<Line>) -> Tree {
    let mut tree = Tree {
        root: Rc::new(Node {
            size: 0,
            children: RefCell::new(vec![]),
        }),
        lookup: HashMap::new(),
    };
    tree.lookup.insert("".to_string(), Rc::clone(&tree.root));
    let mut cwd = "".to_string();
    for line in lines {
        match line {
            Line::Command(Command::Ls) => {}
            Line::Command(Command::Cd(name)) => match name.as_str() {
                "/" => cwd = "".to_string(),
                ".." => {
                    cwd = cwd
                        .rsplit_once('/')
                        .unwrap_or((cwd.as_str(), ""))
                        .0
                        .to_string()
                }
                name => cwd = format!("{cwd}/{name}"),
            },
            Line::Entry(Entry::Dir(name)) => {
                let path = format!("{cwd}/{name}");
                let parent = tree.lookup.get_mut(cwd.as_str()).unwrap();
                let node = Rc::new(Node {
                    size: 0,
                    children: RefCell::new(vec![]),
                });
                // TODO Why does this work only in exactly this order (push, then insert)?
                // Doing insert then push complains about tree.lookup being borrowed twice, why?
                //tree.lookup.insert(path, Rc::clone(&node));
                parent.children.borrow_mut().push(Rc::clone(&node));
                // TODO why does this work without Rc::clone()?
                tree.lookup.insert(path, node);
            }
            Line::Entry(Entry::File(_, size)) => {
                let parent = tree.lookup.get_mut(cwd.as_str()).unwrap();
                let entry = Node {
                    size,
                    children: RefCell::new(vec![]),
                };
                parent.children.borrow_mut().push(Rc::new(entry));
            }
        }
    }
    tree
}

fn get_size(node: &Rc<Node>) -> u64 {
    let mut total = 0;
    // This is how you iterate over a RefCell<Vec<...>>
    for child in node.children.borrow().iter() {
        total += child.size;
        total += get_size(child);
    }
    total
}

fn get_size_at(tree: &Tree, path: &str) -> Option<u64> {
    let node = match path {
        "/" => &tree.root,
        path => tree.lookup.get(&path.to_string())?,
    };
    Some(get_size(node))
}

fn main() {
    let input = include_str!("../input.txt");

    let lines = read_input(input).unwrap();
    let tree = build_tree(lines);

    let mut result = 0;
    for dir in tree.lookup.keys() {
        let size = get_size_at(&tree, dir.as_str()).unwrap();
        if size <= 100000 {
            result += size;
        }
    }
    println!("part 1: {result}");

    let space_used = get_size(&tree.root);
    let space_available = 70000000 - space_used;
    let min_size = 30000000 - space_available;
    let mut result = space_used;
    for dir in tree.lookup.keys() {
        let size = get_size_at(&tree, dir.as_str()).unwrap();
        if size >= min_size && size < result {
            result = size;
        }
    }
    println!("part 2: {result}");
}
