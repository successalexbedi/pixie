use std::sync::{Arc, Mutex};
use std::thread;
use tiny_http::{Server, Response, Header};
use super::protocol::BoxSnapshot;

const VIEWER_HTML: &str = r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Pixie Live Preview</title>
<style>
body{margin:0;background:#eee;font-family:sans-serif;overflow:auto}
#root{position:relative;width:100vw;min-height:100vh}
.pxbox{position:absolute;box-sizing:border-box}
</style></head>
<body><div id="root"></div>
<script>
async function tick() {
  try {
    const res = await fetch('/snapshot.json');
    const tree = await res.json();
    const root = document.getElementById('root');
    root.innerHTML = '';
    // FLAT paint, not nested — every box already carries its true
    // absolute (root-relative) position, computed by the real
    // engine. Nesting DOM elements here would make CSS re-apply
    // position:absolute relative to each parent, double-counting
    // an offset that's already absolute.
    paint(tree, root);
  } catch (e) { console.error(e); }
  setTimeout(tick, 300);
}
function paint(node, root) {
  const el = document.createElement('div');
  el.className = 'pxbox';
  el.style.left = node.x + 'px';
  el.style.top = node.y + 'px';
  el.style.width = node.width + 'px';
  el.style.height = node.height + 'px';
  el.style.background = `rgba(${node.background.join(',')})`;
  if (node.text) {
    el.style.color = `rgba(${node.color.join(',')})`;
    el.style.fontSize = node.font_size + 'px';
    el.textContent = node.text;
  }
  root.appendChild(el);
  for (const child of node.children) paint(child, root);
}
tick();
</script></body></html>"#;

pub struct PreviewServer {
    snapshot: Arc<Mutex<BoxSnapshot>>,
}

impl PreviewServer {
    pub fn start(addr: &str, initial: BoxSnapshot) -> Self {
        let snapshot = Arc::new(Mutex::new(initial));
        let server_snapshot = Arc::clone(&snapshot);
        let server = Server::http(addr).expect("failed to bind preview server");

        thread::spawn(move || {
            for request in server.incoming_requests() {
                let response = match request.url() {
                    "/snapshot.json" => {
                        let json = serde_json::to_string(&*server_snapshot.lock().unwrap())
                            .unwrap_or_else(|_| "null".to_string());
                        let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                        Response::from_string(json).with_header(header)
                    }
                    _ => {
                        let header = Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
                        Response::from_string(VIEWER_HTML).with_header(header)
                    }
                };
                let _ = request.respond(response);
            }
        });

        PreviewServer { snapshot }
    }

    pub fn update(&self, new_snapshot: BoxSnapshot) {
        *self.snapshot.lock().unwrap() = new_snapshot;
    }
}
