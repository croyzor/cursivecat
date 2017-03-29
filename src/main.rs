extern crate hyper;
extern crate rustc_serialize;
extern crate cursive;

// HTTP library
//use hyper::client;
use rustc_serialize::json;
use cursive::Cursive;
use cursive::views::*;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

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

/*
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
    for q in data {
        let comment = q.comment;
        lst.add_child("fuck",TextView::new(comment));
    }
    app.add_layer(lst);
    app.add_global_callback('q', |a| a.quit());
    app.run();
}

/*
fn show_question(curs: &mut Cursive, q: &Question) {
    let panes = LinearLayout::vertical();
    panes.add_child(TextView::new(q.comment));
    panes.add_child(TextView::new(q.reply));
    curs.add_layer(panes);
}
*/

fn main() {
   
    // Open the JSON file located at path
    let path = Path::new("data.json");
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
    let data: Vec<Question> = match json::decode(&s) {
        Ok(question) => question,
        Err(why) => panic!("Decoding failed: {}", why),
    };
    
    ui(data);
}
