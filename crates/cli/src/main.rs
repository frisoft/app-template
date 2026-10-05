//! A driving adapter: the CLI — this binary is the composition root.
//!
//! The composition root is the **only** place allowed to know about every
//! layer. Its job is to:
//!
//! 1. parse arguments / input,
//! 2. construct the concrete adapters from `infrastructure`, and
//! 3. call an `application` workflow.
//!
//! It contains **no business logic**. Adapters parse, wire and delegate.
//!
//! See `AGENTS.md` for the full architecture contract, rules and recipes.
//! The `Payment` / `Money` snippets in that document are **illustrative
//! examples only** — they are intentionally not part of this template.

use anyhow::Result;

fn main() -> Result<()> {
    // TODO: construct your adapters and call an application workflow, e.g.
    //
    //     let repository = PostgresOrderRepository::connect(&dsn).await?;
    //     let order = application::place_order(command, &repository).await?;
    //
    // The `app` binary currently prints nothing to do because the template
    // ships without a domain model. Replace this with your composition root.
    println!("app-template: no workflows wired yet — see AGENTS.md");
    Ok(())
}
