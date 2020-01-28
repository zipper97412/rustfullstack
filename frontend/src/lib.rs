#![recursion_limit="512"]

use yew::{html, Component, ComponentLink, Html, ShouldRender, html::InputData};

use failure::Error;
use yew::format::Json;
use yew::services::ConsoleService;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

// pub struct Model {
//     link: ComponentLink<Self>,
// }

// pub enum Msg {
//     Click,
// }

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Model { link }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::Click => {}
//         }
//         true
//     }

//     fn view(&self) -> Html {
//         html! {
//             <div>
//                 <button onclick=self.link.callback(|_| Msg::Click)>{ "Click2" }</button>
//             </div>
//         }
//     }
// }




pub struct Model {
	console: ConsoleService,
	ws: Option<WebSocketTask>,
	wss: WebSocketService,
	link: ComponentLink<Self>,
	text: String,                    // text in our input box
	server_data: String,             // data received from the server
}

pub enum Msg {
	Connect,                         // connect to websocket server
	Disconnected,                    // disconnected from server
	Ignore,                          // ignore this message
	TextInput(String),               // text was input in the input box
	SendText,                        // send our text to server
	Received(Result<String, Error>), // data received from server
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Model {
			console: ConsoleService::new(),
			ws: None,
			wss: WebSocketService::new(),
			link: link,
			text: String::new(),
			server_data: String::new(),
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Connect => {
				self.console.log("Connecting");
				let cbout = self.link.callback(|Json(data)| {
					ConsoleService::new().log(&format!("Notification: {:?}", data));
					Msg::Received(data)
				});
				let cbnot = self.link.callback(|input| {
					ConsoleService::new().log(&format!("Notification: {:?}", input));
					match input {
						WebSocketStatus::Closed | WebSocketStatus::Error => {
							Msg::Disconnected
						}
						_ => Msg::Ignore,
					}
				});
				if self.ws.is_none() {
					let task = self.wss.connect("ws://127.0.0.1:8000/ws/", cbout, cbnot.into());
					self.ws = task.ok();
				}
				true
			}
			Msg::Disconnected => {
				self.ws = None;
				true
			}
			Msg::Ignore => {
				false
			}
			Msg::TextInput(e) => {
				self.text = e; // note input box value
				true
			}
			Msg::SendText => {
				match self.ws {
					Some(ref mut task) => {
						task.send(Json(&self.text));
						self.text = "".to_string();
						true // clear input box
					}
					None => {
						false
					}
				}
			}
			Msg::Received(Ok(s)) => {
				self.server_data.push_str(&format!("{}\n", &s));
				true
			}
			Msg::Received(Err(s)) => {
				self.server_data.push_str(&format!("Error when reading data from server: {}\n", &s.to_string()));
				true
			}
		}
    }
    
    fn view(&self) -> Html {
        html! {
            <div>
                // connect button
                <p><button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button></p><br/>
                // text showing whether we're connected or not
                <p>{ "Connected: " } { !self.ws.is_none() } </p><br/>
                // input box for sending text
                <p><input type="text", value=&self.text, oninput=self.link.callback(|e: InputData| Msg::TextInput(e.value))></input></p><br/>
                // button for sending text
                <p><button onclick=self.link.callback(|_| Msg::SendText)>{ "Send" }</button></p><br/>
                // text area for showing data from the server
                <p><textarea value=&self.server_data,></textarea></p><br/>
            </div>
        }
	}
}
