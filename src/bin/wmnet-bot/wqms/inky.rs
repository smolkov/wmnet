
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

/// watch
pub fn watch () {
    let (tx, rx) = std::sync::mpsc::channel();
    // let (tx, rx) = crossbeam_channel::unbounded();
    // let mut watcher: RecommendedWatcher = Watcher::with_channel(tx, Duration::from_secs(2))?;

    // for event in rx.iter() {
    // ...
    // }
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |res| tx.send(res).unwrap()).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(wms.rootdir(), RecursiveMode::Recursive)
        .unwrap();

    for res in rx {
        match res {
            Ok(event) => {
                println!("changed: {:?}", event);
                display.reset(&mut delay).expect("error resetting display");
                display.clear(Color::White);
                let chs = wms.channels()?;
                // for (index, ch) in chs.list.iter().enumerate() {
                //     if index < 8 {
                //         let info = ch.info();
                //         display.draw(
                //             ProFont12Point::render_str(format!("{}", info.value).trim())
                //                 .with_stroke(Some(Color::Red))
                //                 .with_fill(Some(Color::White))
                //                 .translate(Coord::new(CORD[index].0, CORD[index].1))
                //                 .into_iter(),
                //         );
                //         display.draw(
                //             ProFont10Point::render_str(info.label.as_str())
                //                 .with_stroke(Some(Color::Black))
                //                 .with_fill(Some(Color::White))
                //                 .translate(Coord::new(CORD[index].0 + VAL, CORD[index].1))
                //                 .into_iter(),
                //         );
                //     }
                // }
                display.draw(
                    ProFont9Point::render_str(format!("status:{}", network::state()).trim())
                        .with_stroke(Some(Color::Red))
                        .with_fill(Some(Color::White))
                        .translate(Coord::new(60, 83))
                        .into_iter(),
                );
                display.draw(
                    ProFont9Point::render_str(format!("IP:{}", network::hostname()).trim())
                        .with_stroke(Some(Color::Black))
                        .with_fill(Some(Color::White))
                        .translate(Coord::new(10, 93))
                        .into_iter(),
                );
                display.update(&mut delay).expect("error updating display");
                println!("Update...");
                println!("Finished - going to sleep");
                display.deep_sleep().unwrap();
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}