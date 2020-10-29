use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc, mpsc};
use std::collections::HashMap;
use rand::Rng;

pub struct voter {
    entrust: Option<String>,
    ticket: i32,
    host: bool,
    score: i32,
    voted: bool,
    votable: bool,
}

impl voter {
    fn new() -> voter {
        voter {
            entrust: None,
            ticket: 0,
            host: false,
            score: 0,
            voted: false,
            votable: false,
        }
    }
}

fn give_right_to(mut hash:HashMap::<String, voter>, sender: String, receiver: String) -> HashMap::<String, voter> {
    if !hash.contains_key(&receiver) || !hash.contains_key(&sender){
        println!("Erroe: there don't exist such a voter named {} or {}", sender, receiver);
    } else {
        if hash.get_mut(&receiver).unwrap().voted || hash.get_mut(&sender).unwrap().ticket == 0 || !hash.get_mut(&receiver).unwrap().votable {
            println!("Error: the receiver have voted or is unvotable, or the sender has no ticket in stock");
        } else if hash.get_mut(&receiver).unwrap().host || hash.get_mut(&sender).unwrap().host {
            println!("Error: the receiver or the sender is the host");
        } else {
            hash.get_mut(&receiver).unwrap().ticket += hash.get(&sender).unwrap().ticket;
            hash.get_mut(&sender).unwrap().ticket = 0;
            hash.get_mut(&sender).unwrap().votable = false;
            hash.get_mut(&sender).unwrap().voted = true;
        }
    }
    hash
}

/*fn grant_right_to(mut hash: HashMap::<String, voter>, sender: String, receiver: String) -> HashMap::<String, voter> {
    if !hash.contains_key(&receiver) || !hash.contains_key(&sender) {
        println!("Erroe: there don't exist such a voter named {} or {}", sender, receiver);
    } else {
        if !hash.get(&sender).unwrap().host {
            println!("Error: the sender {} is not the host!", sender);
        } else if hash.get(&receiver).unwrap().voted || hash.get(&receiver).unwrap().votable {
            println!("Error: the receiver {} have been granted already or have voted!", receiver);
        } else {
            hash.get_mut(&receiver).unwrap().votable = true;
            hash.get_mut(&receiver).unwrap().ticket = 1;
        }
    }
    hash
}

fn init_grant(mut hash: Arc<Mutex<HashMap::<String, voter>>>, voter_name: Vec::<String>) ->  Arc<Mutex<HashMap::<String, voter>>>{
    //println!("voter_name = {:?}", voter_name);
    for i in 0..voter_name.len() {
        let grant_or_not = rand::thread_rng().gen_range(0,2);
        if grant_or_not == 1 {
            hash.lock().unwrap().get_mut(&voter_name[i]).unwrap().ticket = 1;
            //hash.lock().unwrap().get_mut(&voter_name[i]).unwrap().votable = true;//panic因为hash_voter未进行初始化
            println!("The host grant vote right to {}", voter_name[i]);
        }
    }
    hash
}*/

fn main() {
    let hoster = voter {entrust: None, ticket: 0, host: true, score: 0, votable: false, voted: false, };
    println!("Please set the users number: ");
    let mut hash_voter = Arc::new(Mutex::new(HashMap::<String, voter>::new()));
    let mut input = String::new();
    std::io::stdin()
            .read_line(&mut input)
            .expect("Error: failed to read from stdin");
    let trimmed = input.trim();
    let mut users: u32 = 5;
    match trimmed.parse::<u32>() {
        Ok(i) => users = i,
        Err(_) => println!("Error: your input is not a integer!"),
    };
    let (tx1, rx1) = mpsc::channel();
    //let (tx2, rx2) = mpsc::channel();
    //let (tx3, rx3) = mpsc::channel();
    let counter = Arc::new(Mutex::new(0));
    let mut players = Arc::new(Mutex::new(Vec::<String>::new()));
    let mut players_second = Vec::<String>::new();
    println!("Please set players' name: ");
    let mut input = String::new();
    std::io::stdin()
            .read_line(&mut input)
            .expect("Error: failed to read from stdin");
    for word in input.split_whitespace() {
        players.lock().unwrap().push(word.to_string());
        players_second.push(word.to_string());
    }

    for i in 0..users {
        let players_clone = Arc::clone(&players);
        let voter = voter::new();
        let sender1 = mpsc::Sender::clone(&tx1);
        //let sender2 = mpsc::Sender::clone(&tx2);
        //let sender3 = mpsc::Sender::clone(&tx3);
        let counter = Arc::clone(&counter);
        let hash_voter_clone = Arc::clone(&hash_voter);
        let handle = std::thread::spawn(move || {
            let user_name = players_clone.lock().unwrap().get(0).unwrap().clone();
            hash_voter_clone.lock().unwrap().insert(players_clone.lock().unwrap().remove(0), voter);//此处直接使用[]运算符进行读会所有权报错,pop()方法为&mut self
            let grant_or_not = rand::thread_rng().gen_range(0,2);
            if grant_or_not == 1 {
                if let Some(temp) = hash_voter_clone.lock().unwrap().get_mut(&user_name) {
                    temp.votable = true;
                    temp.ticket = 1;
                }
                println!("The host grant vote right to {}", user_name);
            }
            let action_type = rand::thread_rng().gen_range(0,2);//1: give right to another/ 2:vote to another
            if action_type == 0 {
                let action_object = rand::thread_rng().gen_range(0, users);
                sender1.send(i);
                //sender2.send(action_type);
                //sender3.send(action_object);
                println!("voter {} give right to {}", i, action_object);
            } 
        });
        handle.join();
        //drop(handle);
        println!("thread {} finished!", i);
    }

    println!("hello world!");
    let mut snd = Vec::<u32>::new();
    //let mut action = Vec::<u32>::new();
    //let mut rcv = Vec::<u32>::new();
    let mut iter1 = rx1.iter();
    for _ in 0..users {
        if let Ok(msg) = rx1.recv_timeout(Duration::from_secs(2)) {

        } else {
            println!("time out!");
        }
        /*if let info = iter1.next() {
            snd.push(info.unwrap());
            println!("snd = {:?}", snd);
        } else {
            break;
        }*/
        
    }
    println!("========");
    //for i in rx2 { action.push(i); }
    //for i in rx3 { rcv.push(i); }
    //println!("snd = {:?}", snd);
    //println!("action = {:?}", action);
    //println!("rcv = {:?}", rcv);
    /*for i in 0..users {
        if action[i] == 0 {
            give_right_to(hash_voter, players_second[snd[i]], players_second[rcv[i]]);
        }
    }*/

}