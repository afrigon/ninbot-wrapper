use anyhow::Result;
use reqwest::Client;
use tokio::sync::watch;
use tracing::info;

use crate::arguments::Arguments;
use crate::game::GameState;
use crate::game::StateWatcher;
use crate::ninjabrain::Boat;
use crate::ninjabrain::BotProcess;
use crate::ninjabrain::EventStream;
use crate::ninjabrain::InformationMessages;
use crate::ninjabrain::Release;
use crate::ninjabrain::Stronghold;
use crate::overlay::FrameWriter;
use crate::overlay::Line;
use crate::overlay::Renderer;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub struct Service {
    arguments: Arguments
}

impl Service {
    pub fn new(arguments: Arguments) -> Self {
        Self { arguments }
    }

    pub async fn run(self) -> Result<()> {
        let client = Client::builder().user_agent(USER_AGENT).build()?;
        let renderer = Renderer::new()?;
        let mut writer = FrameWriter::new(self.arguments.output)?;
        writer.write(renderer.render(&Line::default())?)?;

        let jar = Release::new(client.clone())?.jar().await?;
        let mut bot = BotProcess::spawn(&jar)?;

        let (stronghold_sender, mut stronghold) = watch::channel(Stronghold::default());
        let (boat_sender, mut boat) = watch::channel(Boat::default());
        let (messages_sender, mut messages) = watch::channel(InformationMessages::default());
        let (state_sender, mut state) = watch::channel(None::<GameState>);

        tokio::spawn(EventStream::new(client.clone(), "/stronghold").run(stronghold_sender));
        tokio::spawn(EventStream::new(client.clone(), "/boat").run(boat_sender));
        tokio::spawn(EventStream::new(client, "/information-messages").run(messages_sender));
        let _watcher = StateWatcher::start(self.arguments.state, state_sender)?;

        let terminated = Self::terminated();
        tokio::pin!(terminated);

        loop {
            let line = Line::new(
                &stronghold.borrow_and_update(),
                &boat.borrow_and_update(),
                &messages.borrow_and_update(),
                state.borrow_and_update().as_ref()
            );
            writer.write(renderer.render(&line)?)?;

            tokio::select! {
                status = bot.wait() => {
                    info!("Ninjabrain Bot exited with {}", status?);
                    break;
                }
                result = &mut terminated => {
                    result?;
                    info!("terminated");
                    break;
                }
                Ok(()) = stronghold.changed() => {}
                Ok(()) = boat.changed() => {}
                Ok(()) = messages.changed() => {}
                Ok(()) = state.changed() => {}
            }
        }

        writer.write(renderer.render(&Line::default())?)
    }

    #[cfg(unix)]
    async fn terminated() -> Result<()> {
        use tokio::signal::unix::SignalKind;
        use tokio::signal::unix::signal;

        let mut terminate = signal(SignalKind::terminate())?;
        let mut interrupt = signal(SignalKind::interrupt())?;

        tokio::select! {
            _ = terminate.recv() => {}
            _ = interrupt.recv() => {}
        }

        Ok(())
    }

    #[cfg(not(unix))]
    async fn terminated() -> Result<()> {
        Ok(tokio::signal::ctrl_c().await?)
    }
}
