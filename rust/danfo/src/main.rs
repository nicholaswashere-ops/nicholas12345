use std::io::{self, Write};

#[derive(Clone, PartialEq)]
enum Scene {
    Start,
    DanfoRide,
    OkadaRide,
    BoltCheck,
    ConductorTrouble,
    CheckpointDelay,
    LekkiArrival,
    Ending,
}

struct Choice {
    label: &'static str,
    tag: &'static str,
    next: Scene,
    cash_delta: i32,
    time_delta: i32,
    vibe_delta: i32,
}

struct SceneData {
    loc: &'static str,
    text: &'static str,
    speaker: &'static str,
    line: &'static str,
    choices: Vec<Choice>,
}

struct GameState {
    time: i32,
    cash: i32,
    vibe: i32,
}

impl GameState {
    fn new() -> Self {
        GameState { time: 90, cash: 2500, vibe: 60 }
    }

    fn vibe_label(&self) -> &'static str {
        if self.vibe >= 70 {
            "Unbothered"
        } else if self.vibe >= 40 {
            "Calm"
        } else if self.vibe >= 20 {
            "Vexed"
        } else {
            "Don Vex Finish"
        }
    }

    fn print_status(&self) {
        println!(
            "\n[ Time left: {} min | Cash: NGN {} | Vibe: {} ]\n",
            self.time.max(0),
            self.cash.max(0),
            self.vibe_label()
        );
    }

    fn apply(&mut self, c: &Choice) {
        self.cash = (self.cash + c.cash_delta).max(0);
        self.time = (self.time + c.time_delta).max(0);
        self.vibe = (self.vibe + c.vibe_delta).clamp(0, 100);
    }
}

fn scene_data(scene: &Scene) -> SceneData {
    match scene {
        Scene::Start => SceneData {
            loc: "OJOTA BUS STOP -- 9:14am",
            text: "Your sister's wedding starts by 11am for Lekki. Traffic don already tie for \
                   Third Mainland. A conductor dey shout \"Lekki-Ajah, one chance!\" from inside a \
                   rickety yellow danfo, while an okada man dey signal you from the other side.",
            speaker: "Conductor",
            line: "\"Enter now now, we dey commot!\"",
            choices: vec![
                Choice {
                    label: "Jump inside the danfo",
                    tag: "Cheaper, but slow -- Third Mainland go choke",
                    next: Scene::DanfoRide,
                    cash_delta: -300,
                    time_delta: -25,
                    vibe_delta: 0,
                },
                Choice {
                    label: "Take the okada instead",
                    tag: "Faster, riskier, costs more",
                    next: Scene::OkadaRide,
                    cash_delta: -800,
                    time_delta: -10,
                    vibe_delta: 0,
                },
                Choice {
                    label: "Wait and check your phone for a Bolt",
                    tag: "Safer, but surge pricing dey dis hour",
                    next: Scene::BoltCheck,
                    cash_delta: 0,
                    time_delta: -8,
                    vibe_delta: 0,
                },
            ],
        },
        Scene::DanfoRide => SceneData {
            loc: "INSIDE THE DANFO -- Third Mainland Bridge",
            text: "The bus dey packed tight, and one aunty beside you don already start to \
                   complain about \"this country.\" Conductor dey hang for door, collecting fare \
                   with speed wey no be here. Bridge dey slow-slow crawl.",
            speaker: "Aunty beside you",
            line: "\"My brother, na so dis country be. Endure am.\"",
            choices: vec![
                Choice {
                    label: "Strike up conversation to pass time",
                    tag: "Small vibe boost",
                    next: Scene::ConductorTrouble,
                    cash_delta: 0,
                    time_delta: -6,
                    vibe_delta: 8,
                },
                Choice {
                    label: "Put on headphones and zone out",
                    tag: "Ignore the go-slow stress",
                    next: Scene::ConductorTrouble,
                    cash_delta: 0,
                    time_delta: -6,
                    vibe_delta: 4,
                },
            ],
        },
        Scene::OkadaRide => SceneData {
            loc: "OKADA -- weaving through Ozumba Mbadiwe",
            text: "Wind dey slap your face as the okada man dodge between danfos and potholes \
                   like professional. You dey enjoy the speed small, until una see a police \
                   checkpoint ahead -- the rider slow down and start counting change quietly.",
            speaker: "Okada man",
            line: "\"Oga, no worry, na small something we go give them.\"",
            choices: vec![
                Choice {
                    label: "Let him handle the checkpoint his way",
                    tag: "Costs a little, keeps moving",
                    next: Scene::LekkiArrival,
                    cash_delta: -500,
                    time_delta: -8,
                    vibe_delta: 0,
                },
                Choice {
                    label: "Insist on stopping properly and asking questions",
                    tag: "Slower, but you keep your money",
                    next: Scene::CheckpointDelay,
                    cash_delta: 0,
                    time_delta: -15,
                    vibe_delta: 0,
                },
            ],
        },
        Scene::BoltCheck => SceneData {
            loc: "OJOTA BUS STOP -- checking your phone",
            text: "Bolt app dey show NGN 4,200 for a ride to Lekki because of surge. Your data \
                   dey also finish small small as the app struggle to load. A danfo conductor \
                   still dey shout beside you, and an okada man just pull up asking if you need \
                   a ride.",
            speaker: "Okada man",
            line: "\"Oga you go still dey here when I don reach Lekki o.\"",
            choices: vec![
                Choice {
                    label: "Book the Bolt anyway -- comfort matters today",
                    tag: "Expensive but stress-free",
                    next: Scene::LekkiArrival,
                    cash_delta: -4200,
                    time_delta: -15,
                    vibe_delta: 0,
                },
                Choice {
                    label: "Forget it, jump on the okada",
                    tag: "Cheaper and faster",
                    next: Scene::OkadaRide,
                    cash_delta: 0,
                    time_delta: -4,
                    vibe_delta: 0,
                },
            ],
        },
        Scene::ConductorTrouble => SceneData {
            loc: "THIRD MAINLAND BRIDGE -- still crawling",
            text: "The conductor dey argue with a passenger about change -- \"I don give you your \
                   balance na, abeg no dull me today.\" The whole bus dey drag am into the \
                   matter. Meanwhile, traffic just start moving small.",
            speaker: "You (thinking)",
            line: "\"If I miss this wedding because of NGN 50 change wahala, e go pain me.\"",
            choices: vec![
                Choice {
                    label: "Stay quiet, focus on the road ahead",
                    tag: "Keep your calm",
                    next: Scene::LekkiArrival,
                    cash_delta: 0,
                    time_delta: -10,
                    vibe_delta: 5,
                },
                Choice {
                    label: "Help settle the change dispute yourself",
                    tag: "Small cash loss, but bus moves faster",
                    next: Scene::LekkiArrival,
                    cash_delta: -100,
                    time_delta: -5,
                    vibe_delta: 10,
                },
            ],
        },
        Scene::CheckpointDelay => SceneData {
            loc: "POLICE CHECKPOINT -- Ozumba Mbadiwe",
            text: "An officer leans in, checks your bag lazily, asks where you dey rush to. You \
                   explain say na wedding. He waves you through with a warning, but you don \
                   already lose precious minutes.",
            speaker: "Officer",
            line: "\"Wedding? Ehen, go well. Tell them make dem no forget rice o.\"",
            choices: vec![Choice {
                label: "Continue to Lekki",
                tag: "",
                next: Scene::LekkiArrival,
                cash_delta: 0,
                time_delta: -5,
                vibe_delta: 0,
            }],
        },
        Scene::LekkiArrival => SceneData {
            loc: "LEKKI -- approaching the wedding venue",
            text: "You can hear highlife music and generator hum from a few streets away. Your \
                   outfit dey slightly rumpled, your phone dey buzz with messages from your \
                   mother, but you don almost reach.",
            speaker: "Mother (text message)",
            line: "\"Where are you?? They are about to start the dance entry o!!\"",
            choices: vec![Choice {
                label: "See how it plays out",
                tag: "",
                next: Scene::Ending,
                cash_delta: 0,
                time_delta: 0,
                vibe_delta: 0,
            }],
        },
        Scene::Ending => SceneData {
            loc: "",
            text: "",
            speaker: "",
            line: "",
            choices: vec![],
        },
    }
}

fn print_ending(state: &GameState) {
    let won = state.time > 0;
    let (title, text): (&str, &str) = if !won {
        (
            "YOU MISSED THE ENTRY DANCE",
            "Time finish before you reach. You arrive just as they're cutting the cake -- your \
             mother is not amused, but at least you made it before the food finished.",
        )
    } else if state.vibe >= 60 {
        (
            "SMOOTH LANDING",
            "You walk in cool, calm, and right on time -- dancing your way straight into the \
             reception like Lagos traffic never stood a chance.",
        )
    } else if state.cash <= 500 {
        (
            "BROKE BUT BLESSED",
            "You made it on time, but your wallet is crying. Worth it though -- you didn't miss \
             your sister's big entrance.",
        )
    } else {
        (
            "YOU MADE IT -- BARELY",
            "Sweaty, slightly late, but present. Your sister spots you from across the room and \
             just shakes her head, smiling.",
        )
    };

    println!("\n===============================");
    println!("{}", title);
    println!("===============================");
    println!("{}\n", text);
}

fn read_choice(max: usize) -> usize {
    loop {
        print!("> ");
        io::stdout().flush().ok();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        match input.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= max => return n - 1,
            _ => println!("Enter a number between 1 and {}.", max),
        }
    }
}

fn main() {
    println!("=====================================");
    println!(" DANFO CHRONICLES");
    println!(" Get across Lagos before your sister's wedding starts. E no easy.");
    println!("=====================================");

    let mut state = GameState::new();
    let mut current = Scene::Start;

    loop {
        if current == Scene::Ending {
            print_ending(&state);
            break;
        }

        state.print_status();
        let data = scene_data(&current);

        println!("-- {} --", data.loc);
        println!("{}\n", data.text);
        if !data.speaker.is_empty() {
            println!("{}: {}\n", data.speaker, data.line);
        }

        for (i, c) in data.choices.iter().enumerate() {
            if c.tag.is_empty() {
                println!("  {}) {}", i + 1, c.label);
            } else {
                println!("  {}) {}  [{}]", i + 1, c.label, c.tag);
            }
        }

        let idx = read_choice(data.choices.len());
        let chosen_next = data.choices[idx].next.clone();
        state.apply(&data.choices[idx]);

        if state.time <= 0 && chosen_next != Scene::Ending {
            current = Scene::Ending;
        } else {
            current = chosen_next;
        }
    }

    println!("Play again? Run the program again to try a different route through Lagos.");
}
