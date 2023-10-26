use std::{io, path::Path, path::PathBuf};

use bstr::io::BufReadExt;

pub fn path_from_bytes(
    path_bytes: &[u8],
) -> Result<PathBuf, std::str::Utf8Error> {
    let path_str = std::str::from_utf8(path_bytes)?;
    Ok(PathBuf::from(path_str))
}

pub fn paths_from_reader<R: io::Read>(rdr: R) -> io::Result<Vec<PathBuf>> {
    let mut paths = vec![];
    let mut line_number = 0;
    io::BufReader::new(rdr).for_byte_line(|line| {
        line_number += 1;
        match path_from_bytes(line) {
            Ok(path) => {
                paths.push(path);
                Ok(true)
            }
            Err(err) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{}: {}", line_number, err),
            )),
        }
    })?;
    Ok(paths)
}

pub fn paths_from_path<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let path = path.as_ref();
    let file = std::fs::File::open(path).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("{}: {}", path.display(), err),
        )
    })?;
    paths_from_reader(file).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("{}:{}", path.display(), err),
        )
    })
}

pub fn paths_from_stdin() -> io::Result<Vec<PathBuf>> {
    let stdin = io::stdin();
    let locked = stdin.lock();
    paths_from_reader(locked).map_err(|err| {
        io::Error::new(io::ErrorKind::Other, format!("<stdin>:{}", err))
    })
}
