use std::fs;

use rouille::post_input;
use rouille::router;
use rouille::try_or_400;
use rouille::Request;
use rouille::Response;

fn main() {
    rouille::start_server("0.0.0.0:80", move |request| {
        router!(request,
            (POST) (/connect) => {
                let data = try_or_400!(post_input!(request, {
                    ssid: String,
                    passkey: String
                }));
                let info = format!("ssid: {}\n psk: {}", data.ssid, data.passkey);
                fs::write("wifi.txt", info).expect("error");
                return Response::redirect_303("/done");

            },
            (GET) (/) => {
                Response::html(r#"
                    <form action="/connect" method="POST">
                        <input type="text" name="ssid" placeholder="WiFi Name" />
                        <input type="text" name="passkey" placeholder="WiFi Password" />
                        <button type="submit">Go!</button>
                    </form>
                "#)
            },
            (GET) (/done) => {
                Response::text("Done!")
            },
            _ => Response::redirect_303("/")
        )
    });
}
