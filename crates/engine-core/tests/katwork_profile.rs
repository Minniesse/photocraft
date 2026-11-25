//! The KatWork build profile: built without the `plugins` feature (no WebAssembly plug-in host), every
//! `plugin.*` command is unknown, and a document that names a plug-in smart filter opens and draws as
//! an upstream build without that plug-in installed does. `cargo test -p photocraft-engine-core
//! --no-default-features --test katwork_profile` runs both tests; the default build runs the second.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use photocraft_engine_core::Session;
use photocraft_engine_core::doc::{LayerContent, SmartFilter};
use serde_json::json;

/// A document with one painted layer, made a smart object.
fn smart_doc() -> Session {
    let mut s = photocraft_engine_core::session();
    s.execute("file.new", json!({"width": 24, "height": 16, "depth": 8})).unwrap();
    s.execute("layer.new.layer", json!({})).unwrap();
    s.edit("pattern", |doc, active| {
        let surf = doc.layer_mut(active.unwrap()).unwrap().surface_mut().unwrap();
        for y in 0..16 {
            for x in 0..24 {
                let v = ((x * 5 + y * 3) % 17) as f32 / 16.0;
                surf.write_pixel(x, y, &[v, 1.0 - v, 0.5, 1.0]);
            }
        }
        Ok(())
    })
    .unwrap();
    s.execute("layer.smartObjects.convertToSmartObject", json!({})).unwrap();
    s
}

/// The active layer's pixels after rasterizing it.
fn rasterized(mut s: Session) -> Vec<f32> {
    s.execute("layer.smartObjects.rasterize", json!({})).unwrap();
    let d = s.active().unwrap();
    d.doc.layer(d.active_layer.unwrap()).unwrap().surface().unwrap().read_region(photocraft_geom::Rect::new(0, 0, 24, 16))
}

#[cfg(not(feature = "plugins"))]
#[test]
fn plugin_commands_are_unknown() {
    let mut s = smart_doc();
    let wasm = "AGFzbQEAAAA="; // the 8-byte WebAssembly header, base64
    let calls = [
        ("plugin.install", json!({"dataBase64": wasm})),
        ("plugin.install", json!({"data": wasm})),
        ("plugin.install", json!({"path": "/tmp/x.wasm"})),
        ("plugin.list", json!({})),
        ("plugin.remove", json!({"id": "org.example.x"})),
        ("plugin.reload", json!({})),
        ("plugin.run", json!({"id": "org.example.x"})),
    ];
    for (id, p) in calls {
        match s.execute(id, p) {
            Err(photocraft_engine_core::EngineError::UnknownCommand(named)) => assert_eq!(named, id),
            other => panic!("{id}: expected the unknown-command error, got {other:?}"),
        }
    }
    assert!(photocraft_engine_core::command_specs().iter().all(|c| !c.id.starts_with("plugin.")), "no plugin.* command is registered");
}

/// A smart filter that names a plug-in (`plugin.run`) does not panic and leaves the pixels as they
/// were, as when its plug-in isn't installed. Runs in both builds: with the host no plug-in is
/// installed here, so both builds must draw the same.
#[test]
fn a_plugin_smart_filter_draws_as_without_the_plugin() {
    let plain = rasterized(smart_doc());
    let mut s = smart_doc();
    s.edit("plug-in filter", |doc, active| {
        let LayerContent::Smart(sm) = &mut doc.layer_mut(active.unwrap()).unwrap().content else { panic!("not a smart object") };
        sm.smart_filters.push(SmartFilter {
            command: "plugin.run".into(),
            params: json!({"id": "org.example.not-installed", "params": {"amount": 3}}),
            blend: photocraft_engine_core::doc::BlendMode::Normal,
            opacity: 1.0,
            visible: true,
        });
        Ok(())
    })
    .unwrap();
    // Hide and show it again: each re-renders the smart object through its filters.
    s.execute("layer.smartFilter.setVisible", json!({"index": 0, "visible": false})).unwrap();
    s.execute("layer.smartFilter.setVisible", json!({"index": 0, "visible": true})).unwrap();
    {
        let d = s.active().unwrap();
        let LayerContent::Smart(sm) = &d.doc.layer(d.active_layer.unwrap()).unwrap().content else { panic!("not a smart object") };
        assert_eq!(sm.smart_filters.len(), 1, "the document keeps the plug-in filter");
        assert_eq!(sm.smart_filters[0].command, "plugin.run");
    }
    assert_eq!(rasterized(s), plain);
}
