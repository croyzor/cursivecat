#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate cursive;

// HTTP library
use hyper::client;
use hyper::rt::{self, Future, Stream};
use cursive::Cursive;
use cursive::views::*;
use cursive::align::*;
use cursive::traits::*;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use std::io::Read;

#[derive(Deserialize)]
struct Response {
    posts: Vec<Question>,
    username: String
}


#[derive(Deserialize)]
struct Question {
    timestamp: i64,
    reply: Option<String>,
    comment: String,
}

fn fetch_user_questions() -> impl Future<Item=(), Error=()> {
    let http_client = client::Client::new();
    let uri = "http://curiouscat.me/api/v2/profile?username=shaun_jen".parse::<hyper::Uri>().unwrap();
    let mut resp = String::new();
    http_client
        .get(uri)
        .and_then(|res| {
            res.into_body().for_each(|chunk| {
                std::io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("exmaplea {}", e))
            })
        })
        .map(|a| println!("done"))
        .map_err(|err| eprintln!("Error {}", err))
}

/// Returns a vertical split with the text and response for a given question
fn show_question(q: &Question) -> LinearLayout {
    let mut panes = LinearLayout::vertical();
    let left  = TextView::new(q.comment.to_string()).with_id("question");
    let reply = match q.reply {
        Some(ref x) => x.to_string(),
        None => "".to_string(),
    };
    let right = TextView::new(reply).with_id("reply");
    // Add question on top
    panes.add_child(left);
    //
    // Add padding
    panes.add_child(DummyView);
    panes.add_child(TextView::new("---").h_align(HAlign::Center));
    panes.add_child(DummyView);

    // Add answer on bottom
    panes.add_child(right);

    return panes;
}

/// The list of questions which lives on the left pane
fn question_list(data: &Vec<Question>) -> SelectView<Question> {
    let mut lst = SelectView::new();
    let mut ix = 0;
    for q in data {
        let mut s: String = "".to_string();
        let c: String = match q.comment.lines().next() {
            Some(s) => s.to_string(),
            None => panic!("Something went wrong in `lines()`"),
        };
        // If length of the question text is too long, cut it off
        if c.len() > 30 {
            s += &c[0..27];
            s += "...";
        }
        else {
            s += &c;
        }
        // Here's some outrageous hackery in case you were wondering whether
        // this was my first time with rust (it is, and I am bad)
        let question_reply = match q.reply {
                Some(ref r) => r.to_string(),
                None    => "".to_string(),
        };
        let mut question = Question {
            comment: q.comment.to_string(),
            reply: Some(question_reply),
            timestamp: q.timestamp
        };
        lst.add_item(s, question);
        ix += 1;
    }
    return lst;
}

/// Shows a specific question
/// Used to update the display when user switches between questions
fn update(curs: &mut Cursive, q: &Question) {
    // TODO: implement line wrapping
    match curs.find_id::<TextView>("question") {
        Some(mut txt) => txt.set_content(q.comment.to_string()),
        None => (),
    };
    let reply = match q.reply {
        Some(ref r) => r.to_string(),
        None => "".to_string(),
    };
    match curs.find_id::<TextView>("reply") {
        Some(mut txt) => txt.set_content(reply),
        None => (),
    };
}

fn test_ui(qs: Vec<Question>) {
    let mut app = Cursive::ncurses();
    let mut lin = LinearLayout::horizontal();
    let mut ql = question_list(&qs);
    let mut qa = show_question(&qs[0]);
    // Function to update textview when a new question is highlighted
    ql.set_on_select(update);
    lin.add_child(ql);
    lin.add_child(DummyView);
    lin.add_child(show_question(&qs[0]));
    app.add_global_callback('q', |a| a.quit());
    app.add_layer(lin);
    app.run();
}

/*
fn main() {
    // Open the JSON file located at path
    let path = Path::new("src/resp.json");
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open file: {}", why),
        Ok(file) => file,
    };

    // Read the file to string s
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read string: {}", why),
        Ok(_) => (),
    };

    // Decode the JSON into a vector of our question struct
    let data: Response = match serde_json::from_str(&s) {
        Ok(question) => question,
        Err(why) => panic!("Decoding failed: {}", why),
    };

    //ui(data.posts);
    test_ui(data.posts);
}
 */

fn main() {
    rt::run(fetch_user_questions());
}
