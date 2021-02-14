use path::{Path, PathBuf};

use super::error::Error;
use crate::token::{self, Token, TokenKind};
use std::{iter::Peekable, path, rc::Rc, todo, vec::IntoIter};

pub fn preprocessor(tokens: Vec<Token>) -> Result<Vec<Token>, Error> {
    preprocessor_impl(tokens)
}

fn preprocessor_impl(tokens: Vec<Token>) -> Result<Vec<Token>, Error> {
    let mut result = Vec::with_capacity(tokens.len());
    let mut iter = tokens.into_iter().peekable();
    while let Some(token) = iter.next() {
        if !is_hash(&token) {
            result.push(token);
            continue;
        }

        // single #
        if let Some(x) = iter.peek() {
            if x.is_bol {
                continue;
            }
        }

        if let Some(token) = iter.peek() {
            if let TokenKind::Ident(ident) = &token.kind {
                match ident.name.as_str() {
                    "include" => {
                        let filepath = token.filepath.clone();

                        let base_path = filepath
                            .chars()
                            .rev()
                            .skip_while(|c| c != &'/')
                            .collect::<String>()
                            .chars()
                            .rev()
                            .collect::<String>();
                        let base_path = Path::new(&base_path);

                        iter.next();
                        result.append(&mut include(&mut iter, base_path)?)
                    }
                    _ => return Err(Error::invalid_preprocessor(token.clone())),
                }
            }
        }
    }
    Ok(result)
}

fn is_hash(token: &Token) -> bool {
    if let TokenKind::HashMark = &token.kind {
        token.is_bol
    } else {
        false
    }
}

fn include(iter: &mut Peekable<IntoIter<Token>>, base_path: &Path) -> Result<Vec<Token>, Error> {
    let base_dirs = vec![
        base_path,
        Path::new("/usr/lib/gcc/x86_64-linux-gnu/8/include"),
        Path::new("/usr/local/include"),
        Path::new("/usr/lib/gcc/x86_64-linux-gnu/8/include-fixed"),
        Path::new("/usr/include/x86_64-linux-gnu"),
        Path::new("/usr/include"),
    ];

    if let Some(x) = iter.next() {
        if let TokenKind::String(filepath) = x.kind {
            if let Some(x) = iter.peek() {
                if !x.is_bol {
                    return Err(Error::invalid_preprocessor(x.clone()));
                }
            }
            let filepath = Path::new(filepath.trim());
            let pathes = base_dirs
                .into_iter()
                .map(|base| base.join(filepath))
                .collect::<Vec<PathBuf>>();

            match find_include_file(pathes) {
                Some(path) => {
                    let path = Rc::new(path.to_str().unwrap().to_string());
                    return match token::tokenize_file(path.clone()) {
                        Ok(stream) => Ok(stream.tokens),
                        Err(e) => Err(Error::todo(e.pos, e.input, path)),
                    };
                }
                None => todo!(),
            }
        }
    }
    todo!();
}

fn find_include_file(pathes: Vec<PathBuf>) -> Option<PathBuf> {
    for path in pathes {
        if path.exists() {
            return Some(path);
        }
    }
    None
}
