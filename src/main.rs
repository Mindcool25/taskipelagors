use std::{error::Error, io::{self, Write}};
use archipelago_rs::{self as ap};
use serde::{Serialize, Deserialize};
use termcolor::{self, StandardStream, WriteColor};

#[derive(Serialize, Deserialize, Debug)]
struct SlotData {
	tasks: Vec<String>,
	rewards: Vec<String>,
	reward_types: Vec<String>,
	task_prereqs: Vec<String>,
	reward_prereqs: Vec<String>,
	lock_prereqs: bool,
	death_link_pool: Vec<String>,
	death_link_weights: Vec<f32>,
	death_link_amnesty: f32,
	death_link_enabled: bool,
	base_reward_location_id: i64,
	base_complete_location_id: i64,
	base_item_id: i64,
	sent_item_names: Vec<String>,
	sent_player_names: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut header_color: termcolor::ColorSpec = termcolor::ColorSpec::new();
	header_color.set_fg(Some(termcolor::Color::Green)).set_bold(true);

	let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
	stdout.set_color(&header_color)?;
	writeln!(&mut stdout, "Welcome to the Taskipelago Alternate Client")?;

	stdout.reset()?;

	let stdin = io::stdin();

	let mut url = String::new();
	write!(&mut stdout, "AP URL: ")?;
	stdout.flush()?;
	stdin.read_line(&mut url)?;

	let mut name = String::new();
	write!(&mut stdout, "Slot: ")?;
	stdout.flush()?;
	stdin.read_line(&mut name)?;

	writeln!(&mut stdout, "Connecting to {url} with name {name}")?;

	let mut client: ap::Client<SlotData> = futures::executor::block_on(
		ap::Client::connect(
			url.trim(),
			name.trim(),
			Some("Taskipelago"),
			ap::ConnectionOptions::new().receive_items(ap::ItemHandling::None)
		)
	)?;

	// writeln!(&mut stdout, "{:?}", client.slot_data())?;

	writeln!(&mut stdout, "Connected!")?;

	let mut command = String::new();

	loop {
		stdout.set_color(&header_color)?;
		writeln!(&mut stdout, "Your tasks:")?;
		stdout.reset()?;
		for (idx, task) in client.slot_data().tasks.iter().enumerate() {
			writeln!(&mut stdout, "{idx:4}. {task}")?;
		}

		write!(&mut stdout, "Check which one? ")?;
		stdout.flush()?;
		stdin.read_line(&mut command)?;
		if let Ok(c) = command.trim().parse::<i64>() {
			client.mark_checked(
				[
					client.slot_data().base_reward_location_id + c,
					client.slot_data().base_complete_location_id + c,
				]
			)?;
		}
	}

	Ok(())
}
