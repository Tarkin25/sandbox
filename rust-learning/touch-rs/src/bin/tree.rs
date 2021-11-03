use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;
use std::io;

const BOTTOM_CORNER: char = '\u{2514}';
const VERTICAL_STRAIGHT: char = '\u{2502}';
const VERTICAL_CROSS_RIGHT: char = '\u{251C}';
const HORIZONTAL_STRAIGHT: char = '\u{2500}';
const SPACE: char = ' ';

fn tree_initial<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut parent_indices = HashMap::new();

    tree(path.as_ref(), 0, false, &mut parent_indices)
}

fn tree(path: &Path, index: i32, is_last: bool, parent_indices: &mut HashMap<i32, bool>) -> io::Result<()> {

    if index > 0 {
        for i in 0..index-1 {
            let display_vertical_straight = !*parent_indices.get(&i).unwrap_or(&false);

            if display_vertical_straight {
                print!("{}", VERTICAL_STRAIGHT);
            } else {
                print!("{}", SPACE);
            }

            print!("{0}{0}", SPACE);
        }

        let corner = if is_last {BOTTOM_CORNER} else {VERTICAL_CROSS_RIGHT};

        print!("{}{}{}", corner, HORIZONTAL_STRAIGHT, SPACE);
    }

    println!("{}", path.file_name().unwrap().to_str().unwrap());

    if path.is_dir() {
        let children = fs::read_dir(path)?;

        let mut children = children.peekable();

        if children.peek().is_some() {
            parent_indices.insert(index-1, is_last);
        }

        while let Some(child) = children.next() {
            let child = child?;
            let path = child.path();
            let child_is_last = children.peek().is_none();

            tree(path.as_path(), index +1, child_is_last, parent_indices)?;
        }
    }

    Ok(())
}

fn main() {
    let mut args = env::args();
    args.next();

    let result = if let Some(root_path) = args.next() {
        tree_initial(root_path)
    } else {
        tree_initial(env::current_dir().expect("Unable to obtain current working directory"))
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
