use image::ImageFormat;
use std::{
    fs,
    io::{prelude::*, BufReader, Cursor, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "custom-hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /newpage HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /muamua HTTP/1.1" => ("HTTP/1.1 200 OK", "muamua.html"),
        "GET /muamua.png HTTP/1.1" => ("HTTP/1.1 200 OK", "muamua.png"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    // let contents = fs::read_to_string(filename).unwrap();
    let contents = if filename.ends_with(".png") {
        let image_data = fs::read(filename).unwrap();
        let image = image::load_from_memory(&image_data).unwrap();
        let mut image_buffer = Vec::new();

        // use temporary cursor as a seekable buffer
        let mut cursor = Cursor::new(Vec::new());
        image.write_to(&mut cursor, ImageFormat::Png).unwrap();
        image_buffer.extend_from_slice(&cursor.into_inner());

        image_buffer
    } else {
        fs::read(filename).unwrap()
    };
    let length = contents.len();

    let response = format!("{}\r\nContent-length: {}\r\n\r\n", status_line, length);
    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(20);
    // 127.0.0.1 is the ip address of computer and 7878 is the port...
    for stream in listener.incoming().take(20) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        }); // creating infinite threads to handle requests concurrently!
    }
}
