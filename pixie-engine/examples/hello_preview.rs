use pixie_node::{div, header, h1, h3, p, button, badge, card};
use pixie_style::{Style, Color, Display, FlexDirection, AlignItems, Sides, px};
use pixie_node::stylesheet::{background, target};
use pixie_engine::layout::TreeBridge;
use pixie_engine::text::new_font_system;
use pixie_engine::preview::protocol::snapshot_tree;
use pixie_engine::preview::server::PreviewServer;
use taffy::prelude::*;
use std::net::UdpSocket;

fn local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}

fn page_style() -> Style {
    let mut s = Style::default();
    s.structural.display = Display::Flex;
    s.structural.flex_direction = FlexDirection::Column;
    s.structural.width = px(800.0);
    s.structural.padding = Sides::all(px(24.0));
    s.structural.row_gap = px(20.0);
    s
}

fn row_style(gap: f32) -> Style {
    let mut s = Style::default();
    s.structural.flex_direction = FlexDirection::Row;
    s.structural.align_items = AlignItems::Center;
    s.structural.column_gap = px(gap);
    s
}

fn main() {
    let mut font_system = new_font_system();

    let tree = div()
        .style(page_style())
        .child((
            header().ref_id("header").child(h1("Pixie Design Playground").ref_id("title")),
            div().ref_id("hero").child(p("Edit this file, save, and see it update — this is your canvas now.").ref_id("hero-text")),
            div().style(row_style(16.0)).child((
                card().ref_id("card-1").child((h3("Fast").ref_id("card-1-title"), p("Real taffy layout, real cosmic-text shaping.").ref_id("card-1-body"))),
                card().ref_id("card-2").child((h3("Typed").ref_id("card-2-title"), p("Styles are Rust structs, not strings.").ref_id("card-2-body"))),
            )),
            div().style(row_style(12.0)).child((
                button("Get Started").ref_id("cta"),
                badge("v0.1").ref_id("version-badge"),
            )),
        ))
        .stylesheet((
            background(Color::rgb(245, 246, 250)),
            target("header").background(Color::rgb(255, 255, 255)).padding(16.0),
            target("title").font_size(28.0).color(Color::hex("#1a1a2e").unwrap()),
            target("hero-text").color(Color::rgb(90, 90, 100)).font_size(16.0),
            target("card-1").background(Color::rgb(255, 255, 255)).radius(12.0).padding(16.0),
            target("card-2").background(Color::rgb(255, 255, 255)).radius(12.0).padding(16.0),
            target("cta").background(Color::hex("#3366ff").unwrap()).color(Color::rgb(255, 255, 255)).radius(8.0).padding(12.0),
            target("version-badge").background(Color::rgb(230, 230, 235)).color(Color::rgb(100, 100, 110)),
        ));

    let mut bridge = TreeBridge::new();
    let root_id = bridge.build(&tree, &mut font_system);
    bridge.compute(
        root_id,
        Size { width: AvailableSpace::Definite(800.0), height: AvailableSpace::MaxContent },
    );

    let snapshot = snapshot_tree(&tree, &bridge, root_id, 0.0, 0.0);

    println!("Serving Pixie live preview:");
    println!("  On this device: http://localhost:8080");
    if let Some(ip) = local_ip() {
        println!("  On your laptop (same WiFi): http://{ip}:8080");
    }

    let _server = PreviewServer::start("0.0.0.0:8080", snapshot);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
