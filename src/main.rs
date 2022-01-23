mod cli;

#[macro_use]
extern crate serde_json;

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use structopt::StructOpt;

fn main() {
  let args = cli::CommandLineArgs::from_args();

  let mut role_definition = match File::open(args.base_definition_filepath) {
    Ok(n) => {
      let reader = io::BufReader::new(n);

      serde_json::from_reader(reader).unwrap()
    }
    Err(_) => {
      json!({
        "description": "Generated by custom-role-generator",
        "name": "<REPLACE_THIS>",
        "AssignableScopes": [
        ],
        "actions": [
        ],
        "notActions": [
        ],
        "dataActions": [],
        "notDataActions": [],
        "isCustom": true
      })
    }
  };

  let re1:Regex = Regex::new("^.* https://[^ ]+ \"([A-Z]+) /subscriptions/([^/]+)/(?:(locations)|(?:resourceGroups/[^/]+/)?providers/([^/]+)/([^/]+)(?:/[^/]+/?([^/]+)?)*)\\?api-version=[0-9]{4}-[0-9]{2}-[0-9]{2} HTTP/1.1\" [0-9]{3} (?:[0-9]+|None)").unwrap();
  let re2:Regex = Regex::new("however, it does not have permission to perform action '([^']+)' on the linked scope\\(s\\) '/subscriptions/([^/]+)/(?:(locations)|(?:resourceGroups/[^/]+/)?providers/([^/]+)/([^/]+)(?:/[^/]+/?([^/]+)?)*)' or the linked scope\\(s\\) are invalid.").unwrap();
  let mut method2action = HashMap::new();

  method2action.insert(String::from("GET"), "read");
  method2action.insert(String::from("PUT"), "write");
  method2action.insert(String::from("PATCH"), "write");
  method2action.insert(String::from("DELETE"), "delete");
  method2action.insert(String::from("POST"), "action");

  let mut vec_actions = vec![String::new(); 0];
  for action in role_definition
    .pointer("/actions")
    .unwrap()
    .as_array()
    .unwrap()
    .iter()
    .map(|s| {
      Regex::new("\"([^\"]+)\"")
        .unwrap()
        .replace(s.as_str().unwrap(), "$1")
    })
  {
    vec_actions.push(action.to_string());
  }
  let mut vec_assignable_scopes = vec![String::new(); 0];

  loop {
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("failed to read from pipe");
    if input == "" {
      break;
    }
    let mut action = String::new();
    let mut assignable_scope = String::new();
    match re1.captures(input.as_str()) {
      Some(caps) => {
        let method = caps.get(1);
        let subscription_id = caps.get(2);
        let subscription_resource = caps.get(3);
        if subscription_resource.is_none() {
          let resource_provider = caps.get(4);
          action.push_str(resource_provider.unwrap().as_str());
          let mut i = 5;
          loop {
            let resource_type = caps.get(i);
            if resource_type.is_none() {
              break;
            }
            action.push_str("/");
            action.push_str(resource_type.unwrap().as_str());
            i = i + 1;
          }
        } else {
          action.push_str("Microsoft.Resources/subscriptions/");
          action.push_str(subscription_resource.unwrap().as_str());
        }
        action.push_str("/");
        action.push_str(
          method2action
            .get(&String::from(method.unwrap().as_str()))
            .unwrap(),
        );
        vec_actions.push(action);

        assignable_scope.push_str("/subscriptions/");
        assignable_scope.push_str(subscription_id.unwrap().as_str());
        vec_assignable_scopes.push(assignable_scope);
      }
      None => match re2.captures(input.as_str()) {
        Some(caps) => {
          let verb = caps.get(1);
          let subscription_id = caps.get(2);
          let subscription_resource = caps.get(3);
          if subscription_resource.is_none() {
            let resource_provider = caps.get(4);
            action.push_str(resource_provider.unwrap().as_str());
            let mut i = 5;
            loop {
              let resource_type = caps.get(i);
              if resource_type.is_none() {
                break;
              }
              action.push_str("/");
              action.push_str(resource_type.unwrap().as_str());
              i = i + 1;
            }
          } else {
            action.push_str("Microsoft.Resources/subscriptions/");
            action.push_str(subscription_resource.unwrap().as_str());
          }
          action.push_str("/");
          action.push_str(verb.unwrap().as_str());
          vec_actions.push(action);
          assignable_scope.push_str("/subscriptions/");
          assignable_scope.push_str(subscription_id.unwrap().as_str());
          vec_assignable_scopes.push(assignable_scope);
        }
        None => {}
      },
    }
  }
  vec_actions.sort();
  vec_actions = vec_actions.into_iter().unique().collect();
  *role_definition.get_mut("actions").unwrap() = json!(vec_actions);

  vec_assignable_scopes.sort();
  vec_assignable_scopes = vec_assignable_scopes.into_iter().unique().collect();
  *role_definition.get_mut("AssignableScopes").unwrap() = json!(vec_assignable_scopes);

  println!(
    "{}",
    serde_json::to_string_pretty(&role_definition).unwrap()
  );
}
