use iron::prelude::*;
use iron::middleware::Handler;
use router::Router;
use urlencoded::UrlEncodedBody;
use std::collections::HashMap;

use config::Config;
use helpers;
use data_access::round::Round as RoundData;
use pusoy_dos::game::game::Game;
use pusoy_dos::cards::types::*;
use pusoy_dos::cards::card::Card;

pub struct GameMove{
    round_data: RoundData,
    hostname: String
}

impl GameMove{

    pub fn new(config:&Config, round_data: RoundData) -> GameMove {
        let hostname = config.get("hostname").unwrap();
        GameMove{ hostname: hostname, round_data: round_data }
    }

    fn execute(&self, user_id:u64, game_id:u64, hand: Vec<Card>) -> Response {
        let round_result = self.round_data.get(game_id);
        match round_result {
            None => {
                info!("redirecting as no round found for game {}", game_id);
                return helpers::redirect(&self.hostname, "games");  // think about an error page here?
            },
            _ => ()
        }

        let round = round_result.unwrap();
        info!("loading game: {}", game_id);

        let game = Game::load(round.clone()).unwrap();
        info!("game loaded");

        let next_player = game.get_next_player().unwrap();
        info!("next player - {:?}", next_player);

        info!("user id : {}", user_id);
        let current_user = game.get_player(user_id).unwrap();
        info!("current user - {:?}", current_user);

        info!("hand: {:?}", hand);
        let valid_move = game.player_move(user_id, hand);
        info!("{:?}", valid_move);

        self.round_data.update_round(game_id, valid_move.unwrap());

        let play_url = format!("play/{}", game_id);
        helpers::redirect(&self.hostname, &play_url)
    }

    fn get_hashmap(&self, req: &mut Request) -> Option<HashMap<String, Vec<String>>> {

        match req.get_ref::<UrlEncodedBody>(){
            Ok(hashmap) => Some(hashmap.to_owned().to_owned()),
            _ => None
        }
    }

    fn get_move(&self, hashmap: Option<HashMap<String, Vec<String>>>) -> Vec<Card>{
        let mut cards = vec!();

        match hashmap {
            Some(h) => {
                for(card, _) in h {
                    cards.push(self.get_card(card));
                }
            },
            _ => ()
        }

        cards
    }

    fn get_card(&self, card:String) -> Card {
        let words = card.split(" ").collect::<Vec<&str>>();
        let rank = self.get_rank(words[1]);
        let suit = self.get_suit(words[0]);
            
        Card::new(rank, suit)
    }

    fn get_rank(&self, rank:&str) -> Rank {
        info!("rank: {}", rank);
        match rank {
            "2"  => Rank::Two,
            "3"  => Rank::Three,
            "4"  => Rank::Four,
            "5"  => Rank::Five,
            "6"  => Rank::Six,
            "7"  => Rank::Seven,
            "8"  => Rank::Eight,
            "9"  => Rank::Nine,
            "10" => Rank::Ten,
            "J"  => Rank::Jack,
            "Q"  => Rank::Queen,
            "K"  => Rank::King,
            "A"  => Rank::Ace,
            _    => panic!("invalid rank supplied in move")
        }
             
    }

    fn get_suit(&self, suit:&str) -> Suit {
        info!("suit: {}", suit);
        match suit {
            "Clubs"    => Suit::Clubs,
            "Hearts"   => Suit::Hearts,
            "Diamonds" => Suit::Diamonds,
            "Spades"   => Suit::Spades,
            _          => panic!("invalid suit supplied in move")
        }

    }

}

impl Handler for GameMove {


    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let ref hashmap = self.get_hashmap(req);

        let ref query = req.extensions.get::<Router>().unwrap().find("id");

        let hand = self.get_move(hashmap.to_owned());

        
        info!("{:?}", hashmap);

        let session_user_id = helpers::get_user_id(req);
        let redirect_to_homepage = helpers::redirect(&self.hostname, "");

        let resp = match session_user_id {
            Some(user_id) => {
                match *query {
                    Some(id) => {
                        self.execute(user_id, id.parse::<u64>().unwrap(), hand)
                    },
                    _ => redirect_to_homepage
                }
            },
            _ => redirect_to_homepage
        };

        Ok(resp)
    }

}

