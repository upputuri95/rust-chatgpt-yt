use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env; // env module for env variables, OpenAi access key
use std::io::{stdin, stdout, Write};


// a struct to work with the api response
struct OAIResponse{
    choices: Vec<OAIChoices>,
    id: Option<String>,
    Object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    Choices: Vec<OAIChoices>
}

// a struct for the choices
#[derive(Deserialize, Debug)]
struct OAIChoices{
    text: String,
    index: u8,
    logprobs: Option<u8>
    finish_reason: String,
}

//a struct for the request you will make to the api

#[derive(Serialize, Debug)]
struct OAIChoices{
    prompt: String,
    max_tokens: u16,
}

//tokio async main function
+[tokio::main]
async fn main() -> Result<(), Box<dyn std:: error:: Error + send + Sync>> {
// load my env variable
    dotenv().ok();
// create a Httpconnector, hyper
    let https = Httpsconnector::new();
// create a client
    let client = Client:: builder().build(https);
// URL to which we will make tthe request
    let uri = "https://api.openai.com/v1/engines/text-davinci=001/completions";
// preamble, prompt to chatGPT
    let preamble = "Generate a sql code for the given statement";
// token in the header, 
    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);
// loop, inside the loop a way to read user input
    loop {
        print!(">");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin()
        .read_line(&mut user_text)
        .expect("Failed to read line");
        println!("");
//spinnner, wait for the response
        let sp =  Spinner::new(&Spinnners::Dosts12, "\t\tOpenAI is Thinking...".into());
//request to chatGPT for every single user input, loop
        let oai_request = oai_Request{
            prompt: format!("{} {}", preamble, user_text),
            max_tokens: 1000,
        };
        let body = Body::from(serde__json::to_vec(&oai_request)?);
        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();
// response and we print the response
        let res = client.request(req).await?;
        let body = hyper::body::aggregate(res).await?;
        let json = OAIResponse = serda_json::from_reader(body.reader())?;
        sp.stop();
        println!("");
        println("{}", json.choices[0].text);
    }
    ok(())
}



