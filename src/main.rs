/*
   Copyright © 2013 Free Software Foundation, Inc
   See licensing in LICENSE file

   File: examples/ex_1.rs
   Author: Jesse 'Jeaye' Wilkerson
   Description:
   Simple "Hello, world" example.
   */



extern crate graph_note;

fn main(){
    let database = graph_note::database::create("test.db");

    graph_note::view::display(database);
}

