#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate hyper_tls;
extern crate cursive;

// HTTP library
use hyper::client;
use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;
use cursive::Cursive;
use cursive::views::*;
use cursive::align::*;
use cursive::traits::*;
use std::string::String;

#[derive(Deserialize)]
struct Response {
    posts: Vec<Question>,
}


#[derive(Deserialize)]
struct Question {
    timestamp: i64,
    reply: Option<String>,
    comment: String,
}

fn fetch_user_questions(url: String) -> impl Future<Item=String, Error=()> {
    let https = HttpsConnector::new(1)
        .expect("TLS initialization failed");
    let http_client = client::Client::builder()
        .build::<_, hyper::Body>(https);

    let uri = url
        .parse::<hyper::Uri>()
        .unwrap();

    http_client
        .get(uri)
        .and_then(|res| {
            println!("response {}", res.status());
            println!("headers {:#?}", res.headers());
            res.into_body().concat2()
        })
        .and_then(|body| {
            let s = ::std::str::from_utf8(&body)
                .expect("httpbin sends utf-8 JSON");
            Ok(s.to_string())
        })
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
    show_question(&qs[0]);
    // Function to update textview when a new question is highlighted
    ql.set_on_select(update);
    lin.add_child(ql);
    lin.add_child(DummyView);
    lin.add_child(show_question(&qs[0]));
    app.add_global_callback('q', |a| a.quit());
    app.add_layer(lin);
    app.run();
}

fn print_usage(prog_name: &String) {
    println!("Usage:");
    println!("    {} <USERNAME>\n", prog_name);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        print_usage(&args[0]);
        return;
    }

    let url = format!("https://curiouscat.me/api/v2/profile?username={}",
                      args[1]);
    let fut = fetch_user_questions(url.to_string())
        .map(|body| {
            // Decode the JSON into a vector of our question struct
            let data: Response = match serde_json::from_str(&body) {
                Ok(question) => question,
                Err(why) => panic!("Decoding failed: {}", why),
            };

            //ui(data.posts);
            test_ui(data.posts);
        });

    rt::run(fut);
}
