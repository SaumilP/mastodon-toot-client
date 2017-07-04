// file responsible for sending toot to Mastodon social site...
extern crate mammut;
extern crate mastodon_bot;
extern crate dotenv;

use mammut::{Data, Mastodon, Registration};
use mammut::status_builder::StatusBuilder;
use mammut::apps::{AppBuilder, Scope};

fn main() {
    dotenv::dotenv().ok();
    if ::std::env::var("CLIENT_ID").is_ok() {
    	println!(">>>>> About to toot using `.env` file ... <<<<<<<<");
        from_configuration();
    } else {
    	println!(">>>>> About to toot from new client registration ... <<<<<<<<");
        register_and_post();
    }
}

#[allow(dead_code)]
fn register_and_post() {
    let app = AppBuilder {
        client_name: "rust-client",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scope::ReadWrite,
        website: None,
    };

	// Register the app using configuration in `.env` file
    let mut registration = Registration::new(env("BASE")).expect("Registration creation failed");
    registration.register(app).expect("Register failed");
    let url = registration.authorise().expect("Registration authorize failed");

    println!("Please visit {}, authorise and enter the code it gives you: ", url);

	// get token code and use that for posting new status...
	
	// FIXME Below code reads code entered by user, find a better way to do website scrapping if required
    let mut code = String::new();
    std::io::stdin().read_line(&mut code).expect("Reading code failed");
    
    // create the app client
    let mastodon = registration.create_access_token(code).expect("Creating access token failed");
    println!("{:#?}", mastodon.data);
    
    // Lets update status
    let status = mastodon_bot::random_location();
    let sb = StatusBuilder::new(status);
    println!("StatusBuilder = {:#?}", sb);
    
    mastodon.new_status(sb).expect("Could not post status");
    
    println!("Done!");
}

#[allow(dead_code)]
fn from_configuration() {
    let data = Data {
        base: env("BASE"),
        client_id: env("CLIENT_ID"),
        client_secret: env("CLIENT_SECRET"),
        redirect: String::from("urn:ietf:wg:oauth:2.0:oob"),
        token: env("TOKEN")
    };

	// create mastodon client from configured data
    let mastodon = Mastodon::from_data(data).expect("Could not create Mastodon instance from data");

	// lets get new random status
    let status = mastodon_bot::random_location();
    println!("Posting {}", status);

	// post status to mastodon
    let sb = StatusBuilder::new(status);
    println!("StatusBuilder = {:#?}", sb);
    mastodon.new_status(sb).expect("Could not post status");

    println!("Done!");
}

fn env(s: &str) -> String {
    ::std::env::var(s).unwrap_or_else(|_| {
    		panic!("must have `{}` definded", s)
    })
}
