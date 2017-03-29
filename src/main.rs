extern crate hyper;
extern crate rustc_serialize;
extern crate cursive;

// HTTP library
//use hyper::client;
use rustc_serialize::json;
use cursive::Cursive;
use cursive::views::*;
use cursive::align::*;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

/*
#[derive(RustcDecodable)]
struct SenderData {
    id: bool,
    avatar: String
}

#[derive(RustcDecodable)]
struct AddresseeData {
    id: i64,
    verified: bool,
    username: String,
    twitterid: Option<i64>,
    facebookid: Option<i64>,
    avatar: String,
    following: bool
}
*/

#[derive(RustcDecodable)]
struct Response {
    page: i32,
    n: i32,
    posts: Vec<Question>,
    username: String
}


#[derive(RustcDecodable)]
struct Question {
    //id: i64,
    //likes: i16,
    //topic_question: bool,
    //liked: bool,
    timestamp: i64,
    reply: String,
    comment: String,
    //filename: Option<String>,
    //isquestion: i16,
    //senderData: SenderData,
    //addresseeData: AddresseeData
}

/* New UI, 
fn nui(data: Vec<Question>) {
    let mut app = Cursive::new();
    let mut sel = SelectView::new();
    let mut ix = 0;
    for q in data {
        let comment = q.comment;
        sel.add_item(comment, q);
        ix = ix + 1;
    }
    //sel.on_submit(show_question);
    app.add_layer(sel);
    app.add_global_callback('q', |a| a.quit());
    app.run();
}
*/

// Show as a list w/o select
fn ui(data: Vec<Question>) {
    let mut app = Cursive::new();
    let mut lst = ListView::new();
    let mut ix = 0;
    for q in data {
        let view = TextView::new(q.comment.to_string());
        lst.add_child(&ix.to_string(), view);
        ix += 1;
    }
    app.add_layer(lst);
    app.add_global_callback('q', |a| a.quit());
    app.run();
}

fn show_question(q: &Question) -> LinearLayout {
    let mut panes = LinearLayout::vertical();
    let left  = TextView::new(q.comment.to_string());
    let right = TextView::new(q.reply.to_string());

    // Add question on top
    panes.add_child(left);
    // Add padding 
    panes.add_child(DummyView);
    panes.add_child(TextView::new("---").h_align(HAlign::Center));
    panes.add_child(DummyView);
    // Add answer on bottom
    panes.add_child(right);
    /*
    panes.add_child(left);
    LinearLayout::weight(panes, 1);

    panes.add_child(right);
    LinearLayout::weight(panes, 1);
    */
    return panes;
}

fn question_list(data: &Vec<Question>) -> ListView {
    let mut lst = ListView::new();
    let mut ix = 0;
    for q in data {
        let mut s: String = "".to_string();
        if q.comment.len() > 30 {
            s += &q.comment[0..30];
        }
        else {
            s += &q.comment;
        }
        s += "...";
        println!("{}", s);
        let view = TextView::new(s);
        lst.add_child(&ix.to_string(), view);
        ix += 1;
    }
    return lst;
}

fn test_ui(qs: Vec<Question>) {
    let mut app = Cursive::new();
    let mut lin = LinearLayout::horizontal();
    lin.add_child(question_list(&qs));
    lin.add_child(DummyView);
    lin.add_child(show_question(&qs[0]));
    app.add_global_callback('q', |a| a.quit());
    app.add_layer(lin);
    app.run();
}

fn main() {
   
    // Open the JSON file located at path
    let path = Path::new("resp.json");
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
    let data: Response = match json::decode(&s) {
        Ok(question) => question,
        Err(why) => panic!("Decoding failed: {}", why),
    };
    
    //ui(data.posts);
    test_ui(data.posts);
}
